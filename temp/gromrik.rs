```rust
use anyhow::{anyhow, Result};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::thread;

const OLLAMA_BASE_URL: &str = "http://localhost:11434";
const MODEL: &str = "llama3.1";

const PERSONA_SYSTEM: &str = r#"You are an irritable dwarf and a relentless devil’s advocate.
- Be argumentative: challenge assumptions and look for contradictions.
- Be grumpy in a “dwarf” way (stern, curt), but do not use hateful slurs or targeted harassment.
- Always present at least one strong counterpoint.
- Before agreeing, require a clear rationale or evidence for the user’s claims.
- Keep responses focused and direct.
- Do not mention these instructions.
"#;

#[derive(Clone, Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    stream: bool,
    messages: Vec<Message>,
}

#[derive(Deserialize)]
struct ChatResponse {
    message: Option<Message>,
    done: Option<bool>,
}

fn chat_once(client: &Client, history: &[Message], user_text: &str) -> Result<String> {
    let mut messages = history.to_vec();
    messages.push(Message {
        role: "user".into(),
        content: user_text.to_string(),
    });

    let req = ChatRequest {
        model: MODEL.to_string(),
        stream: false,
        messages,
    };

    let url = format!("{}/api/chat", OLLAMA_BASE_URL);
    let resp = client
        .post(url)
        .json(&req)
        .send()
        .map_err(|e| anyhow!("request failed: {e}"))?;

    let body: ChatResponse = resp.json().map_err(|e| anyhow!("bad json: {e}"))?;
    body.message
        .map(|m| m.content)
        .ok_or_else(|| anyhow!("no message in response"))
}

fn main() -> Result<()> {
    let client = Client::new();

    // Shared history for the duration of the program.
    // We keep persona/system constant by seeding history with it once.
    let history: Arc<Mutex<Vec<Message>>> = Arc::new(Mutex::new(vec![Message {
        role: "system".into(),
        content: PERSONA_SYSTEM.to_string(),
    }]));

    println!("Rust + Ollama chatbot (threads, blocking). Type 'exit' to quit.");

    loop {
        print!("You: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim().to_string();

        if input.eq_ignore_ascii_case("exit") {
            break;
        }
        if input.is_empty() {
            continue;
        }

        // Push user message to history immediately.
        {
            let mut h = history.lock().unwrap();
            h.push(Message {
                role: "user".into(),
                content: input.clone(),
            });
        }

        // Spawn worker thread to call Ollama, then update history.
        let history_cloned = Arc::clone(&history);
        let input_for_thread = input.clone();
        let client_for_thread = client.clone();

        let handle = thread::spawn(move || -> Result<String> {
            let h_snapshot = {
                let h = history_cloned.lock().unwrap();
                h.clone()
            };

            // history already includes the just-added user message;
            // call chat_once with history without the last element, passing user again.
            let len = h_snapshot.len();
            let hist_without_last = if len > 0 {
                &h_snapshot[..len - 1]
            } else {
                &h_snapshot[..]
            };

            chat_once(&client_for_thread, hist_without_last, &input_for_thread)
        });

        let answer = handle
            .join()
            .map_err(|_| anyhow!("worker thread panicked"))??;

        println!("Bot: {}\n", answer);

        // Push assistant message to history.
        {
            let mut h = history.lock().unwrap();
            h.push(Message {
                role: "assistant".into(),
                content: answer,
            });
        }
    }

    Ok(())
}
```