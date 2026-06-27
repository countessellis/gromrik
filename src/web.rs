use crossbeam_channel::{unbounded,Receiver};

use crate::chat::*;
use crate::config::*;
use crate::defaults::*;

///////////// WEB

#[derive(Clone)]
#[allow(dead_code)]
pub(crate) struct WEB {
  pub(crate) chat: Chat,
  pub(crate) recv: Receiver<StreamEvent>,
}

impl WEB {
  pub(crate) fn new(config: &Config) -> WEB {
    let (send, recv) = unbounded::<StreamEvent>();
    WEB { recv: recv, chat: Chat::new(&config,send) }
  }

  pub(crate) fn run(&mut self) {
    println!("WEB not yet implemented for {}.",APP_NAME);
  }
}
