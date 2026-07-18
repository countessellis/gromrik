use ansi_term::Colour;

use crate::defaults::*;

///////////// Splash

pub(crate) fn version() -> String {
  format!("
{} {}.{} ({})
Copyright (C) {} {}
Licensed under the {} License
  ",APP_NAME,VERSION_ID,BUILD_ID,BUILD_TIME,COPYRIGHT,AUTHORS,LICENSE)
}

pub(crate) fn raw_splash() -> String {
  format!("\n{}\n",TEXT_SPLASH)
}

pub(crate) fn splash() -> String {
  let splash: String =  raw_splash();
  format!("\n{}\n",Colour::RGB(130, 108, 72).bold().paint(splash))
}
