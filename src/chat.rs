use serde::{Deserialize, Serialize};
use std::io::Read;
use std::collections::HashMap;
use std::io;
use std::time::Duration;
use std::io::BufReader;
use std::io::BufRead;

use crate::config::*;
use crate::defaults::*;

pub(crate) struct Chat {
  pub(crate) config:  Config,
  pub(crate) agent:   ureq::Agent,
  pub(crate) history: Vec<Message>,
}

#[derive(Clone,Serialize,Deserialize)]
pub(crate) struct Message {
  pub(crate) role:    String,
  pub(crate) content: String,
}

#[derive(Serialize)]
pub(crate) struct ChatRequest {
  pub(crate) model:    String,
  pub(crate) stream:   bool,
  pub(crate) messages: Vec<Message>,
  pub(crate) options: Option<OllamaOptions>,
}

#[derive(Deserialize)]
pub(crate) struct ChatResponse {
  pub(crate) message: Option<Message>,
  pub(crate) done:    Option<bool>,
}

#[derive(Serialize)]
pub(crate) struct OllamaOptions {
  pub(crate) temperature: f32,
  pub(crate) num_ctx: u32,
  pub(crate) num_predict: i32,
  pub(crate) top_p: f32,
}


impl Chat {
  pub(crate) fn new(config: &Config) -> Chat {
    let agent_config = ureq::config::Config::builder().http_status_as_error(false).timeout_connect(Some(Duration::from_secs(10))).timeout_global(Some(Duration::from_secs(2700))).build();
    let agent = ureq::Agent::new_with_config(agent_config);
    let history: Vec<Message> = vec![Message {
      role: "system".into(),
      content: Self::render(SYSTEM_PROMPT_TEMPLATE,&HashMap::new()),
    }];
    Chat {
      config:  config.clone(),
      agent:   agent,
      history: history,
    }
  }

  pub(crate) fn chat(&mut self) -> Result<Option<&str>,String> {
    println!("\nYou:\n");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
      Ok(_) => {},
      Err(err) => return Err(format!("Failed to get user's message: {}",err)),
    }
    let input = input.trim().to_string();
    if input.eq_ignore_ascii_case("exit") {
      return Ok(Some("exit"));
    }
    if input.is_empty() {
      return Ok(None);
    }
    self.history.push(Message {
      role: "user".into(),
      content: input.clone(),
    });
    let len = self.history.len();
    let hist_without_last = if len > 0 {
      &self.history[..len - 1]
    } else {
      &self.history[..]
    };
    println!("");
    match Self::submit(&self.config,&self.agent,hist_without_last,&input) {
      Ok(answer) => {
        println!("\n{}:\n\n{}\n",APP_NAME,answer);
        self.history.push(Message {
          role: "assistant".into(),
          content: answer,
        });
      },
      Err(err) => eprintln!("{}",err),
    }
    Ok(None)
  }

  pub(crate) fn submit(config: &Config, agent: &ureq::Agent, history: &[Message], user_message: &str) -> Result<String,String> {
    let mut messages = history.to_vec();
    messages.push(Message {
      role: "user".into(),
      content: user_message.to_string(),
    });
    let req = ChatRequest {
      model: config.model.clone(),
      stream: true,
      messages,
      options: Some(OllamaOptions { temperature: 0.75, num_ctx: 2048, num_predict: 150, top_p: 0.9 }),
    };
    match agent.post(&config.llm_server_url).send_json(&req) {
      Ok(mut resp) => {
        if resp.status().is_client_error() || resp.status().is_server_error() {
          let mut err_body: String = String::new();
          match resp.body_mut().as_reader().read_to_string(&mut err_body) {
            Ok(_)   => return Err(format!("Status Code: {}, Response from {}: {}",resp.status(),req.model,err_body)),
            Err(err) => return Err(format!("Status Code: {}, Failed to get body from {}: {}",resp.status(),req.model,err)),
          }
        } else {
          let reader = BufReader::new(resp.body_mut().as_reader());
          let mut complete_response = String::new();
          for line_result in reader.lines() {
            match line_result {
              Ok(line) => {
                if let Ok(chunk) = serde_json::from_str::<ChatResponse>(&line) {
                  if let Some(message) = chunk.message {
                    print!("{}", message.content);
                    std::io::Write::flush(&mut std::io::stdout()).ok();
                    complete_response.push_str(&message.content);
                  }
                }
              },
              Err(err) => return Err(format!("Error reading stream chunk: {}", err)),
            }
          }
          println!();
          if complete_response.is_empty() {
            Err("No message content returned from the stream.".to_string())
          } else {
            Ok(complete_response)
          }
        }
      },
      Err(err) => return Err(format!("Request to {} failed: {}",req.model,err)),
    }
  }

  pub(crate) fn render(template: &str, fields: &HashMap<&str,String>) -> String {
    let mut prompt: String = template.to_string();
    for (key,value) in fields {
      prompt = prompt.replace(&format!("{{{{{}}}}}",key.to_uppercase()),value);
    }
    prompt
  }
}

