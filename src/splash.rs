pub type Colour = nu_ansi_term::Color;

use crate::defaults::*;

///////////// Splash

pub(crate) fn raw_splash() -> String {
  format!("\n{}\n",TEXT_SPLASH)
}

pub(crate) fn splash() -> String {
  let splash: String =  raw_splash();
  format!("\n{}\n",Colour::Rgb(130, 108, 72).bold().paint(splash))
}
