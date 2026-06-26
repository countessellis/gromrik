use serde::{Deserialize, Serialize};
use std::io::Read;
use std::collections::HashMap;
use std::time::Duration;
use std::io::BufReader;
use std::io::BufRead;
use crossbeam_channel::Sender;

use crate::config::*;
use crate::defaults::*;

#[derive(Clone)]
pub(crate) struct Chat {
  pub(crate) config:  Config,
  pub(crate) agent:   ureq::Agent,
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
  pub(crate) model:    String,
  pub(crate) stream:   bool,
  pub(crate) messages: Vec<Message>,
  pub(crate) options: Option<OllamaOptions>,
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
    let history: Vec<Message> = vec![Message {
      role: "system".into(),
      content: Self::render(SYSTEM_PROMPT_TEMPLATE,&HashMap::new()),
    }];
    Chat {
      config:  config.clone(),
      agent:   agent,
      history: history,
      send:    send,
    }
  }

  pub(crate) fn chat(&mut self,input: &str) -> Result<(),String> {
    self.history.push(Message {
      role: "user".into(),
      content: input.to_string(),
    });
    let len = self.history.len();
    let hist_without_last = if len > 0 {
      self.history[..len - 1].to_vec()
    } else {
      self.history.clone()
    };
    let config_clone = self.config.clone();
    let agent_clone  = self.agent.clone();
    let send_clone   = self.send.clone(); 
    let input_clone  = input.to_string();
    std::thread::spawn(move || {
      if let Err(err) = Self::submit(&config_clone,&agent_clone,send_clone,&hist_without_last,&input_clone) {
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
      messages,
      options: Some(OllamaOptions { temperature: 0.75, num_ctx: 2048, top_p: 0.9 }),
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

  pub(crate) fn render(template: &str, fields: &HashMap<&str,String>) -> String {
    let mut prompt: String = template.to_string();
    for (key,value) in fields {
      prompt = prompt.replace(&format!("{{{{{}}}}}",key.to_uppercase()),value);
    }
    prompt
  }
}

