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

impl Default for UI {
  fn default() -> Self {
    let dummy_config = Config::defaults();
    UI::CLI(CLI::new(&dummy_config))
  }
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
      UI::GUI(_) => if let UI::GUI(owned_gui) = std::mem::take(self) { owned_gui.run(); },
      UI::WEB(web) => web.run(),
    }
  }
}
