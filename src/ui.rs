use crate::cli::*;
use crate::config::*;
use crate::gui::*;
use crate::mode::*;
use crate::tui::*;
use crate::web::*;

///////////// UI

#[derive(Clone)]
pub(crate) enum UI {
  CLI(CLI),
  TUI(TUI),
  GUI(GUI),
  WEB(WEB),
}

impl UI {
  pub(crate) fn new(config: &Config) -> UI {
    match config.mode {
      Mode::CLI => UI::CLI(CLI::new(config)),
      Mode::TUI => UI::TUI(TUI::new(config)),
      Mode::GUI => UI::GUI(GUI::new(config)),
      Mode::WEB => UI::WEB(WEB::new(config)),
    }
  }

  pub(crate) fn run(&mut self) {
    match self {
      UI::CLI(cli) => cli.run(),
      UI::TUI(tui) => tui.run(),
      UI::GUI(gui) => gui.run(),
      UI::WEB(web) => web.run(),
    }
  }
}
