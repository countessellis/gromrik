#![cfg(any(feature = "cli", feature = "tui", feature = "gui", feature = "web"))]

#[cfg(feature = "cli")]
use crate::cli::*;
#[cfg(feature = "gui")]
use crate::gui::*;
#[cfg(feature = "tui")]
use crate::tui::*;
#[cfg(feature = "web")]
use crate::web::*;

use crate::config::*;
use crate::defaults::*;
use crate::help::*;
use crate::mode::*;

///////////// UI


#[derive(Clone)]
pub(crate) enum UI {
  #[cfg(feature = "cli")]
  CLI(CLI),
  #[cfg(feature = "tui")]
  TUI(TUI),
  #[cfg(feature = "gui")]
  GUI(GUI),
  #[cfg(feature = "web")]
  WEB(WEB),
  Help(Help),
  None,
}

impl Default for UI {
  fn default() -> Self {
    match DEFAULT_MODE {
      #[cfg(feature = "cli")]
      "cli" => {
        let dummy_config = Config::defaults();
        UI::CLI(CLI::new(&dummy_config))
      },
      #[cfg(feature = "tui")]
      "tui" => {
        let dummy_config = Config::defaults();
        UI::TUI(TUI::new(&dummy_config))
      },
      #[cfg(feature = "gui")]
      "gui" => {
        let dummy_config = Config::defaults();
        UI::GUI(GUI::new(&dummy_config))
      },
      #[cfg(feature = "web")]
      "web" => {
        let dummy_config = Config::defaults();
        UI::WEB(WEB::new(&dummy_config))
      },
      _     => UI::None,
    }
  }
}

impl UI {
  pub(crate) fn new(config: &Config) -> UI {
    match config.mode {
      #[cfg(feature = "cli")]
      Mode::CLI  => UI::CLI(CLI::new(config)),
      #[cfg(feature = "tui")]
      Mode::TUI  => UI::TUI(TUI::new(config)),
      #[cfg(feature = "gui")]
      Mode::GUI  => UI::GUI(GUI::new(config)),
      #[cfg(feature = "web")]
      Mode::WEB  => UI::WEB(WEB::new(config)),
      Mode::Help => UI::Help(Help::new(&config)),
      #[cfg(not(all(feature = "cli",feature = "tui",feature = "gui",feature = "web")))]
      _          => UI::None,
    }
  }

  pub(crate) fn run(&mut self) {
    match self {
      #[cfg(feature = "cli")]
      UI::CLI(cli)   => cli.run(),
      #[cfg(feature = "tui")]
      UI::TUI(tui)   => tui.run(),
      #[cfg(feature = "gui")]
      UI::GUI(_)     => if let UI::GUI(owned_gui) = std::mem::take(self) { owned_gui.run(); },
      #[cfg(feature = "web")]
      UI::WEB(web)   => web.run(),
      UI::Help(help) => println!("{}",help),
      UI::None       => {},
    }
  }
}
