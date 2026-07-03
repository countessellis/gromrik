use std::io::Write;

use crate::ui::*;
use crate::config::*;
use crate::splash::*;

///////////// Run

pub fn run() {
  println!("{}",splash());
  println!("{}",version());
  log::info!("{}",version());
  let config: Config = Config::get_config();
  log::info!("{}",config);
  print!("\x1b]0;{}\x07",config.persona.name);
  std::io::stdout().flush().unwrap();
  #[cfg(any(feature = "cli", feature = "tui", feature = "gui", feature = "web"))]
  {
    let mut ui: UI = UI::new(&config);
    ui.run();
  }
}
