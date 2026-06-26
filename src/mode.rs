use std::{env::{args,Args},fmt,str::FromStr};

use crate::util;

///////////// Mode

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Mode {
  CLI,
  TUI,
  GUI,
  WEB,
}

impl Default for Mode {
  fn default() -> Self {
    let bin_name: String = util::bin_name();
    let bin_name: &str = if let Some(index) = bin_name.find(".") { &bin_name[..index] } else { &bin_name };
    let mode: &str = if let Some(index) = bin_name.find("-") { &bin_name[index+1..] } else { "cli" };
    match mode {
      "cli"  => Mode::CLI,
      "tui"  => Mode::TUI,
      "gui"  => Mode::GUI,
      "web"  => Mode::WEB,
      _      => Mode::CLI,
    }
  }
}

impl FromStr for Mode {
  type Err = &'static str;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "cli"  => Ok(Mode::CLI),
      "tui"  => Ok(Mode::TUI),
      "gui"  => Ok(Mode::GUI),
      "web"  => Ok(Mode::WEB),
      _      => Ok(Default::default()),
    }
  }
}

impl fmt::Display for Mode {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mode: &str = match self {
      Mode::CLI => "cli",
      Mode::TUI => "tui",
      Mode::GUI => "gui",
      Mode::WEB => "web",
    };
    write!(f, "{}",mode)
  }
}

impl Mode {
  pub(crate) fn mode() -> Mode {
    println!("Getting mode from arguments.");
    let mut args: Args = args();
    while let Some(arg) = args.next() {
      match arg.as_str() {
        "--mode" => match args.next() {
          Some(mode) => match Mode::from_str(mode.as_str()) {
            Ok(mode) => return mode,
            Err(_)   => return Default::default(),
          },
          None        => {},
        },
        "--cli" => return Mode::CLI,
        "--tui" => return Mode::TUI,
        "--gui" => return Mode::GUI,
        "--web" => return Mode::WEB,
        _ => {},
      }
    }
    Default::default()
  }
}

