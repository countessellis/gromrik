#![cfg(feature = "web")]

use crossbeam_channel::{unbounded,Receiver};

use crate::chat::*;
use crate::config::*;

///////////// WEB

#[derive(Clone)]
#[allow(dead_code)]
pub(crate) struct WEB {
  pub(crate) config: Config,
  pub(crate) chat:   Chat,
  pub(crate) recv:   Receiver<StreamEvent>,
}

impl WEB {
  pub(crate) fn new(config: &Config) -> WEB {
    let (send, recv) = unbounded::<StreamEvent>();
    WEB {
      config: config.clone(),
      chat: Chat::new(&config,send),
      recv: recv,
    }
  }

  pub(crate) fn run(&mut self) {
    println!("WEB not yet implemented for {}.",self.config.persona.name);
  }
}
