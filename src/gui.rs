use crossbeam_channel::{unbounded,Receiver};

use crate::chat::*;
use crate::config::*;
use crate::defaults::*;

///////////// GUI

#[derive(Clone)]
#[allow(dead_code)]
pub(crate) struct GUI {
  pub(crate) chat: Chat,
  pub(crate) recv: Receiver<StreamEvent>,
}

impl GUI {
  pub(crate) fn new(config: &Config) -> GUI {
    let (send, recv) = unbounded::<StreamEvent>();
    GUI { recv: recv, chat: Chat::new(&config,send) }
  }

  pub(crate) fn run(&mut self) {
    println!("GUI not yet implemented for {}.",APP_NAME);
  }
}
