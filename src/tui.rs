use crossbeam_channel::{unbounded,Receiver};

use crate::chat::*;
use crate::config::*;
use crate::defaults::*;

///////////// TUI

#[derive(Clone)]
#[allow(dead_code)]
pub(crate) struct TUI {
  pub(crate) chat: Chat,
  pub(crate) recv: Receiver<StreamEvent>,
}

impl TUI {
  pub(crate) fn new(config: &Config) -> TUI {
    let (send, recv) = unbounded::<StreamEvent>();
    TUI { recv: recv, chat: Chat::new(&config,send) }
  }

  pub(crate) fn run(&mut self) {
    println!("TUI not yet implemented for {}.",APP_NAME);
  }
}
