use crate::ui::*;
use crate::config::*;
use crate::splash::*;

///////////// Run

pub fn run() {
  println!("{}",splash());
  println!("{}",version());
  let config: Config = Config::get_config();
  let mut ui: UI = UI::new(&config);
  ui.run();
}
