use serde::{Deserialize, Serialize};
use std::io::Read;
use std::collections::HashMap;
use std::time::Duration;
use std::io::BufReader;
use std::io::BufRead;
use crossbeam_channel::Sender;
use crossbeam_channel::unbounded;
#[cfg(any(feature = "tui", feature = "gui", feature = "web"))]
pub use std::fs::{self,File};
#[cfg(any(feature = "tui", feature = "gui", feature = "web"))]
pub use std::io::Write;
#[cfg(any(feature = "tui", feature = "gui", feature = "web"))]
pub use std::path::Path;

use crate::config::*;
use crate::defaults::*;

use crate::util;

#[derive(Clone)]
pub(crate) struct Chat {
  pub(crate) config:  Config,
  pub(crate) agent:   ureq::Agent,
  pub(crate) system:  Message,
  pub(crate) history: Vec<Message>,
  pub(crate) send:    Sender<StreamEvent>,
}

#[derive(Clone,Serialize,Deserialize)]
pub(crate) struct Message {
  pub(crate) role:    String,
  pub(crate) content: String,
}

#[derive(Serialize)]
pub(crate) struct ChatRequest {
  pub(crate) model:      String,
  pub(crate) stream:     bool,
  pub(crate) keep_alive: String,
  pub(crate) messages:   Vec<Message>,
  pub(crate) options:    Option<OllamaOptions>,
}

#[derive(Deserialize)]
pub(crate) struct ChatResponse {
  pub(crate) message: Option<Message>,
  pub(crate) total_duration: Option<u64>,
  pub(crate) eval_duration: Option<u64>,
  pub(crate) prompt_eval_count: Option<u64>,
  pub(crate) eval_count: Option<u64>,
}

#[derive(Serialize)]
pub(crate) struct OllamaOptions {
  pub(crate) temperature: f32,
  pub(crate) num_ctx: u32,
  pub(crate) top_p: f32,
  pub(crate) microstat: u32,
  pub(crate) microstat_eta: f32,
  pub(crate) microstat_tau: f32,
}

#[derive(Clone)]
pub(crate) enum StreamEvent {
  Token(String),
  Finished,
}



impl Chat {
  pub(crate) fn new(config: &Config, send: Sender<StreamEvent>) -> Chat {
    let agent_config = ureq::config::Config::builder().http_status_as_error(false).timeout_connect(Some(Duration::from_secs(10))).timeout_global(Some(Duration::from_secs(2700))).build();
    let agent = ureq::Agent::new_with_config(agent_config);
    let mut chat: Chat = Chat {
      config:  config.clone(),
      agent:   agent.clone(),
      system:  Message {
        role:    "system".into(),
        content: util::render(&config.persona.prompt,&HashMap::new()),
      },
      history: Vec::new(),
      send:    send,
    };
    if config.persona.name == "Commoner".to_string() {
      log::info!("Generating commoner details.");
      println!("Generating commoner details.\n");
      let time_index = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as usize % COMMONER_TRAITS.len();
      let random_trait = COMMONER_TRAITS[time_index];
      let fields: HashMap<&str,String> = HashMap::from([
        ("TRAIT",random_trait.to_string()),
      ]);
      let (send, recv) = unbounded::<StreamEvent>();
      let config_clone = config.clone();
      let agent_clone  = agent.clone();
      std::thread::spawn(move || {
        if let Err(err) = Self::submit(&config_clone,&agent_clone,send,&Vec::new(),&util::render(COMMONER_INIT_PROMPT,&fields)) {
          log::error!("Submit execution failed in background thread: {}", err);
        }
      });
      let mut response: String = String::new();
      while let Ok(event) = recv.recv() {
        match event {
          StreamEvent::Token(token) => {
            response.push_str(&token);
          }
          StreamEvent::Finished => break,
        }
      }
      log::debug!("Results of commoner init: {}",response);
      if let Some((name_part, bio_part)) = response.split_once('|') {
        let name: String = name_part.replace("NAME:", "").trim().to_string();
        let bio:  String = bio_part.replace("BACKSTORY:", "").trim().to_string();
        log::info!("Commoner Name: {}",name);
        log::info!("Commoner Backstory: {}",bio);
        let fields: HashMap<&str,String> = HashMap::from([
          ("NAME",name.clone()),
          ("BACKSTORY",bio.clone()),
        ]);
        chat.config.persona.name = name.clone();
        chat.system = Message {
          role:    "system".to_string(),
          content: util::render(&config.persona.prompt,&fields),
        };
        chat.history = vec![Message {
          role:    "assistant".into(),
          content: config.persona.greeting.clone(),
        }];
      } else {
        log::warn!("Failed to initialize commoner, it will be easily confused: Output invalid: {}",response);
        chat.system = Message {
          role:    "system".to_string(),
          content: "You are generic fantasy human commoner.".to_string(),
        };
        chat.history = vec![Message {
          role:    "assistant".into(),
          content: config.persona.greeting.clone(),
        }];
      }
    } else {
      chat.history = vec![Message {
        role:    "assistant".into(),
        content: config.persona.greeting.clone(),
      }];
    }
    chat
  }

  pub(crate) fn chat(&mut self,input: &str) -> Result<(),String> {
    let new_msg = Message {
      role: "user".into(),
      content: input.to_string(),
    };
    let len = self.history.len();
    let messages: Vec<Message> = vec![
      self.system.clone(),
      Message {
        role: "system".into(),
        content: format!("[OLD MEMORY LOG - ONLY reference this if the user uses pronouns like 'instead' or 'that'\n{}",self.history.clone().into_iter().map(|msg| format!("{}: {}",msg.role,msg.content)).collect::<Vec<String>>().join("\n")),
      },
    ];
    self.history.push(new_msg.clone());
    let config_clone = self.config.clone();
    let agent_clone  = self.agent.clone();
    let send_clone   = self.send.clone(); 
    let input_clone  = input.to_string();
    std::thread::spawn(move || {
      if let Err(err) = Self::submit(&config_clone,&agent_clone,send_clone,&messages,&input_clone) {
        log::error!("Submit execution failed in background thread: {}", err);
      }
    });
    Ok(())
  }

  pub(crate) fn submit(config: &Config, agent: &ureq::Agent, send: Sender<StreamEvent>, history: &[Message], user_message: &str) -> Result<String,String> {
    let mut messages = history.to_vec();
    messages.push(Message {
      role: "user".into(),
      content: user_message.to_string(),
    });
    let req = ChatRequest {
      model: config.model.clone(),
      stream: true,
      keep_alive: "30m".to_string(),
      messages,
      options: Some(OllamaOptions { temperature: 0.85, num_ctx: 8192, top_p: 0.9, microstat: 2, microstat_eta: 0.5, microstat_tau: 4.5 }),
    };
    match agent.post(&config.llm_server_url).header("Connection", "close").send_json(&req) {
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
          let mut final_metrics: Option<ChatResponse> = None;
          for line_result in reader.lines() {
            match line_result {
              Ok(line) => {
                if let Ok(chunk) = serde_json::from_str::<ChatResponse>(&line) {
                  if let Some(ref message) = chunk.message {
                    let _ = send.send(StreamEvent::Token(message.content.clone()));
                    complete_response.push_str(&message.content);
                  }
                  if chunk.eval_count.is_some() {
                    final_metrics = Some(chunk);
                    break; 
                  }
                }
              },
              Err(err) => return Err(format!("Error reading stream chunk: {}", err)),
            }
          }
          if let Some(metrics) = final_metrics {
            let total_sec = metrics.total_duration.unwrap_or(0) as f64 / 1_000_000_000.0;
            let eval_sec = metrics.eval_duration.unwrap_or(0) as f64 / 1_000_000_000.0;
            let prompt_tokens = metrics.prompt_eval_count.unwrap_or(0);
            let output_tokens = metrics.eval_count.unwrap_or(0);
            let tokens_per_sec = if eval_sec > 0.0 { output_tokens as f64 / eval_sec } else { 0.0 };
            log::info!("Metrics -> Prompt Tokens: {} | Output Tokens: {} | Speed: {:.2} t/s | Gen Time: {:.2}s | Total Time: {:.2}s",prompt_tokens,output_tokens,tokens_per_sec,eval_sec, total_sec);
          }
          let _ = send.send(StreamEvent::Finished);
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

  #[cfg(any(feature = "tui",feature = "gui",feature = "web"))]
  pub(crate) fn save_history(&self, filename: &String, history_lines: &Vec<(String, String)>) -> Result<(), String> {
    if let Some(parent_dir) = Path::new(&filename).parent() {
      if let Err(err) = fs::create_dir_all(parent_dir) {
        log::error!("Failed to create parent directory structure for history file: {}",err);
        return Err(format!("Failed to create directory structure for history file: {}",err));
      } 
    }
    let mut file = match File::create(filename) {
      Ok(file) => file,
      Err(err) => return Err(format!("Failed to create save history to {}: {}",filename,err)),
    };
    let json = match serde_json::to_string_pretty(history_lines) {
      Ok(json) => json,
      Err(err) => return Err(format!("Failed to convert history to JSON: {}",err)),
    };
    match file.write_all(json.as_bytes()) {
      Ok(())   => Ok(()),
      Err(err) => return Err(format!("Failed to write history to {}: {}",filename,err)),
    }
  }

  #[cfg(any(feature = "tui",feature = "gui",feature = "web"))]
  pub(crate) fn load_history(&mut self, filename: &String) -> Result<Vec<(String, String)>, String> {
    let mut file = match File::open(filename) {
      Ok(file) => file,
      Err(err) => return Err(format!("No saved history file found at {}: {}",filename,err)),
    };
    let mut json = String::new();
    match file.read_to_string(&mut json) {
      Ok(_)    => {},
      Err(err) => return Err(format!("Failed to read history from {}: {}",filename,err)),
    }
    let lines: Vec<(String, String)> = match serde_json::from_str(&json) {
      Ok(lines) => lines,
      Err(err)  => return Err(format!("Saved history file {} is corrupted: {}",filename,err)),
    };
    self.history.clear();
    for (sender, content) in &lines {
       let role = if sender == "You" {
         "user".into()
       } else if sender == "System" {
         continue;
       } else {
         "assistant".into()
       };
       self.history.push(Message { role, content: content.clone() });
    }
    Ok(lines)
  }
}

