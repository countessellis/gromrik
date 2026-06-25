use crate::chat::*;
use crate::config::*;
use crate::defaults::*;
use crate::splash::*;

///////////// Run

pub fn run() {
  println!("{}",splash());
  println!("{}",version());
  let config: Config = Config::get_config();
  let mut chat: Chat = Chat::new(&config);
  println!("
-----------------------------------
  {} AI. Type 'exit' to quit.
-----------------------------------
",APP_NAME);
  loop {
    match chat.chat() {
      Ok(Some(direction)) => match direction {
        "exit" => break,
        _      => continue,
      },
      Ok(None) => continue,
      Err(err) => eprintln!("Client returned error: {}",err),
    }
  }
}
