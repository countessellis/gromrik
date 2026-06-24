use crate::chat::*;
use crate::config::*;
use crate::splash::*;

///////////// Run

pub fn run() {
  println!("{}",splash());
  println!("{}",version());
  let config: Config = Config::get_config();
  println!("{}",config);
  match Chat::chat(&config) {
    Ok(())   => {},
    Err(err) => eprintln!("Client returned error: {}",err),
  }
}
