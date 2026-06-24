use serde::{Deserialize, Serialize};
use std::io::Read;
use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::time::Duration;

use crate::config::*;
use crate::defaults::*;

pub(crate) struct Chat;

#[derive(Clone,Serialize,Deserialize)]
pub(crate) struct Message {
  pub(crate) role:    String,
  pub(crate) content: String,
}

#[derive(Clone,Serialize)]
pub(crate) struct ChatRequest {
  pub(crate) model:    String,
  pub(crate) stream:   bool,
  pub(crate) messages: Vec<Message>,
}

#[derive(Clone,Deserialize)]
pub(crate) struct ChatResponse {
  pub(crate) message: Option<Message>,
  pub(crate) done:    Option<bool>,
}

impl Chat {
  pub(crate) fn chat(config: &Config) -> Result<(),String> {
    let agent_config = ureq::config::Config::builder().http_status_as_error(false).timeout_connect(Some(Duration::from_secs(10))).timeout_global(Some(Duration::from_secs(2700))).build();
    let agent = ureq::Agent::new_with_config(agent_config);
    let mut history: Vec<Message> = vec![Message {
      role: "system".into(),
      content: Self::render(SYSTEM_PROMPT_TEMPLATE,&HashMap::new()),
    }];

    println!("{} AI Chatbot. Type 'exit' to quit.",APP_NAME);

    loop {
      print!("You: ");
      match io::stdout().flush() {
        Ok(()) => {},
        Err(err) => return Err(format!("Failed to get user's message: {}",err)),
      }
      let mut input = String::new();
      match io::stdin().read_line(&mut input) {
        Ok(_) => {},
        Err(err) => return Err(format!("Failed to get user's message: {}",err)),
      }
      let input = input.trim().to_string();
      if input.eq_ignore_ascii_case("exit") {
        break;
      }
      if input.is_empty() {
        continue;
      }
      history.push(Message {
        role: "user".into(),
        content: input.clone(),
      });
      let len = history.len();
      let hist_without_last = if len > 0 {
        &history[..len - 1]
      } else {
        &history[..]
      };
      match Self::submit(&config,&agent,hist_without_last,&input) {
        Ok(answer) => {
          println!("{}: {}\n",APP_NAME,answer);
          history.push(Message {
            role: "assistant".into(),
            content: answer,
          });
        },
        Err(err) => eprintln!("{}",err),
      }
    }
    Ok(())
  }

  pub(crate) fn submit(config: &Config, agent: &ureq::Agent, history: &[Message], user_message: &str) -> Result<String,String> {
    let mut messages = history.to_vec();
    messages.push(Message {
      role: "user".into(),
      content: user_message.to_string(),
    });
    let req = ChatRequest {
      model: config.model.clone(),
      stream: false,
      messages,
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
          match resp.body_mut().read_json::<ChatResponse>() {
            Ok(resp) => match resp.message {
              Some(message) => Ok(message.content),
              None          => Err(format!("No message returned.")),
            },
            Err(err) => return Err(format!("Failed to parse response from {} failed: {}",req.model,err)),
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

