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
  let mut ui: UI = UI::new(&config);
  ui.run();
}
