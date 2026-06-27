use crossbeam_channel::{unbounded,Receiver};

use crate::chat::*;
use crate::config::*;
use crate::defaults::*;

///////////// CLI

#[derive(Clone)]
pub(crate) struct CLI {
  pub(crate) chat: Chat,
  pub(crate) recv: Receiver<StreamEvent>,
}

impl CLI {
  pub(crate) fn new(config: &Config) -> CLI {
    let (send, recv) = unbounded::<StreamEvent>();
    CLI { recv: recv, chat: Chat::new(&config,send) }
  }

  pub(crate) fn run(&mut self) {
    println!("
-----------------------------------
  {} AI. Type 'exit' to quit.
-----------------------------------
",APP_NAME);
    println!("\n{}  {}:\n\n  {}",GROMRIK_EMOJI,APP_NAME,DEFAULT_GREETING);
    loop {
      let indent = "  "; // 4 spaces indentation
      let max_width = 80;   // Wrap at 80 characters
      println!("\n{} You:\n",HUMAN_EMOJI);
      print!("{}",indent);
      std::io::Write::flush(&mut std::io::stdout()).ok();
      let mut input = String::new();
      if let Err(err) = std::io::stdin().read_line(&mut input) {
        log::error!("Failed to get user's message: {}", err);
        println!("\n{}  {}:\nI can't even hear you!\n",GROMRIK_EMOJI,APP_NAME);
        continue;
      }
      let input = input.trim().to_string();
      if input.eq_ignore_ascii_case("exit") {
        break;
      }
      if input.is_empty() {
        continue;
      }
      if let Err(err) = self.chat.chat(&input) {
        log::error!("Engine error: {}", err);
        println!("\n{}  {}:\nBah! Leave me alone!\n",GROMRIK_EMOJI,APP_NAME);
        continue;
      }
      println!("\n{}  {}:\n",GROMRIK_EMOJI,APP_NAME);
      print!("{}", indent);
      std::io::Write::flush(&mut std::io::stdout()).ok();
      let mut complete_answer = String::new();
      let mut current_column = indent.len();
      while let Ok(event) = self.recv.recv() {
        match event {
          StreamEvent::Token(token) => {
            complete_answer.push_str(&token);
            
            for ch in token.chars() {
              if ch == '\n' {
                print!("\n{}", indent);
                current_column = indent.len();
              } else {
                if ch.is_whitespace() && current_column >= max_width {
                  print!("\n{}", indent);
                  current_column = indent.len();
                } else {
                  print!("{}", ch);
                  current_column += 1;
                }
              }
            }
            std::io::Write::flush(&mut std::io::stdout()).ok();
          }
          StreamEvent::Finished => break,
        }
      }
      println!();
      if !complete_answer.is_empty() {
        self.chat.history.push(Message {
          role: "assistant".into(),
          content: complete_answer,
        });
      }
    }
  }
}
