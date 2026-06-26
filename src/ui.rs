use crate::cli::*;
use crate::config::*;
use crate::mode::*;

///////////// UI

#[derive(Clone)]
pub(crate) enum UI {
  CLI(CLI),
  TUI,
  GUI,
  WEB,
}

impl UI {
  pub(crate) fn new(config: &Config) -> UI {
    match config.mode {
      Mode::CLI => UI::CLI(CLI::new(config)),
      Mode::TUI => UI::TUI,
      Mode::GUI => UI::GUI,
      Mode::WEB => UI::WEB,
    }
  }

  pub(crate) fn run(&mut self) {
    match self {
      UI::CLI(cli) => cli.run(),
      _ => {},
    }
  }
}
