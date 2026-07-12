use env_logger::TimestampPrecision;
use log::LevelFilter;

use crate::logger::LogTarget;

// Build Constants:

pub(crate) const APP_NAME:    &str = "Gromrik";
pub(crate) const BUILD_NAME:  &str = env!("BUILD_NAME");
pub(crate) const VERSION_ID:  &str = env!("CARGO_PKG_VERSION");
pub(crate) const BUILD_TIME:  &str = env!("BUILD_TIME");
pub(crate) const BUILD_ID:    &str = env!("BUILD_ID");
pub(crate) const AUTHORS:     &str = env!("CARGO_PKG_AUTHORS");
pub(crate) const COPYRIGHT:   &str = "2026";
pub(crate) const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
pub(crate) const LICENSE:     &str = env!("CARGO_PKG_LICENSE");


// Logging Constants:
pub(crate) const DEFAULT_LOG_LEVEL:            LevelFilter                                 = LevelFilter::Debug; // LevelFilter::Info;
pub(crate) const DEFAULT_LOG_FORMAT_TIMESTAMP: Option<env_logger::fmt::TimestampPrecision> = Some(TimestampPrecision::Millis); // None;
pub(crate) const DEFAULT_LOG_FORMAT_LEVEL:     bool                                        = true; // false;
pub(crate) const DEFAULT_LOG_FORMAT_TARGET:    bool                                        = true; // false;
pub(crate) const DEFAULT_LOG_DIR:              &'static str                                = "logs";
pub(crate) const DEFAULT_LOG_FILE:             &'static str                                = "gromrik.log";
//pub(crate) const DEFAULT_LOG_TARGET:           env_logger::fmt::Target                     = env_logger::fmt::Target::Stdout;
pub(crate) const DEFAULT_LOG_TARGET:           LogTarget                                   = LogTarget::File(DEFAULT_LOG_DIR, DEFAULT_LOG_FILE);
pub(crate) const REDACT_LOG_LIST:              &str                                        = "config/redact_list.txt";


// Path Constants:

pub(crate) const DEFAULT_CONFIG_FILE:  &str = "config/gromrik.cfg";

#[cfg(any(feature = "tui",feature = "gui",feature = "web"))]
pub(crate) const DEFAULT_HISTORY_FILE: &str = "history/history.json";


// Control Constants:

pub(crate) const DEFAULT_MODE:           &str = {
  #[cfg(feature = "gui")]
  {
    "gui"
  }
  #[cfg(all(feature = "tui",not(feature = "gui")))]
  {
    "tui"
  }
  #[cfg(all(feature = "cli",not(feature = "tui"),not(feature = "gui")))]
  {
    "cli"
  }
  #[cfg(all(feature = "web",not(feature = "cli"),not(feature = "tui"),not(feature = "gui")))]
  {
    "web"
  }
  #[cfg(all(not(feature = "web"),not(feature = "cli"),not(feature = "tui"),not(feature = "gui")))]
  {
    "help"
  }
};
pub(crate) const MODE_LIST:             &[&str] = &[
  #[cfg(feature = "cli")]
  { 
    "cli"
  },
  #[cfg(feature = "tui")]
  { 
    "tui"
  },
  #[cfg(feature = "gui")]
  { 
    "gui"
  },
  #[cfg(feature = "web")]
  { 
    "web"
  },
  { 
    "help"
  }
];
pub(crate) const DEFAULT_LLM_SERVER_URL: &str = "http://localhost:11434/api/chat";
pub(crate) const DEFAULT_MODEL:          &str = "qwen2.5:7b-instruct-q4_K_M";
#[cfg(any(feature = "cli",feature = "tui"))]
pub(crate) const HUMAN_EMOJI:            &str = "\u{1F914}\u{1F4AC}";


// General Persona Constants:

pub(crate) const DEFAULT_PERSONA: &str = {
  #[cfg(feature = "gromrik")]
  {
    "gromrik"
  }
  #[cfg(all(feature = "lyranis",not(feature = "gromrik")))]
  {
    "lyranis"
  }
  #[cfg(all(not(feature = "lyranis"),not(feature = "gromrik")))]
  {
    "commoner"
  }
};
#[cfg(all(feature = "gromrik", feature = "lyranis"))]
pub(crate) const PERSONA_LIST: &str = "gromrik, lyranis, commoner";
#[cfg(all(feature = "gromrik", not(feature = "lyranis")))]
pub(crate) const PERSONA_LIST: &str = "gromrik, commoner";
#[cfg(all(feature = "lyranis", not(feature = "gromrik")))]
pub(crate) const PERSONA_LIST: &str = "lyranis, commoner";
#[cfg(not(any(feature = "gromrik", feature = "lyranis")))]
pub(crate) const PERSONA_LIST: &str = "commoner";

// Gromrik Persona Constants:

#[cfg(feature = "gromrik")]
pub(crate) const GROMRIK_GREETING:  &str = "Bah! Why are you bothering me?";
#[cfg(feature = "gromrik")]
pub(crate) const GROMRIK_DISMISSAL: &str = "Go away!";
#[cfg(feature = "gromrik")]
pub(crate) const GROMRIK_EMOJI:     &str = "\u{1F624}\u{26Cf}\u{FE0F}";

// Lyranis Persona Constants:

#[cfg(feature = "lyranis")]
pub(crate) const LYRANIS_GREETING:  &str = "I wonder... Oh! Fine greetings, stranger. Do you want something?";
#[cfg(feature = "lyranis")]
pub(crate) const LYRANIS_DISMISSAL: &str = "Now what was I contemplating again...?";
#[cfg(feature = "lyranis")]
pub(crate) const LYRANIS_EMOJI:     &str = "\u{1F9DD}\u{1F52E}";

// Commoner Persona Constants:
pub(crate) const COMMONER_TRAITS:        &[&str] = &["rustic", "weathered", "cheerful", "anxious", "rebellious", "humble", "curious", "shrewd"];
pub(crate) const COMMONER_GREETING:      &str    = "Hello.";
pub(crate) const COMMONER_DISMISSAL:      &str    = "What did you say, I missed that.";
pub(crate) const COMMONER_EMOJI:         &str    = "\u{1F9D1}";
#[cfg(any(feature = "gui",feature = "web"))]
pub(crate) const COMMONER_FULL_IMAGE:    &[u8]   = &[ 137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 1, 0, 0, 0, 1, 8, 6, 0, 0, 0, 31, 21, 196, 137, 0, 0, 0, 10, 73, 68, 65, 84, 8, 29, 99, 0, 1, 0, 0, 5, 0, 1, 138, 109, 188, 32, 0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130 ];
#[cfg(feature = "tui")]
pub(crate) const COMMONER_TEXT_IMAGE:    &str    = "\u{1F9D1}";


// General Includes:

pub(crate) const HELP_TEMPLATE: &str  = include_str!("../resources/help.tpl");
#[cfg(any(feature = "gui",feature = "web"))]
pub(crate) const PERSONA_FONT:  &[u8] = include_bytes!("../resources/MedievalSharp-Regular.ttf");
#[cfg(any(feature = "gui",feature = "web"))]
pub(crate) const DIALOGUE_FONT: &[u8] = include_bytes!("../resources/EBGaramond-VariableFont_wght.ttf");

// Gromrik Persona Includes:

#[cfg(feature = "gromrik")]
pub(crate) const GROMRIK_PROMPT:     &str  = include_str!("../resources/gromrik/gromrik_prompt.tpl");
#[cfg(all(feature = "gromrik",any(feature = "gui", feature = "web")))]
pub(crate) const GROMRIK_FULL_IMAGE: &[u8] = include_bytes!("../resources/gromrik/gromrik-full.png");
#[cfg(all(feature = "gromrik",feature = "tui"))]
pub(crate) const GROMRIK_TEXT_IMAGE: &str  = include_str!("../resources/gromrik/gromrik2.txt");

// Lyranis Persona Includes:

#[cfg(feature = "lyranis")]
pub(crate) const LYRANIS_PROMPT:     &str  = include_str!("../resources/lyranis/lyranis_prompt.tpl");
#[cfg(all(feature = "lyranis",any(feature = "gui", feature = "web")))]
pub(crate) const LYRANIS_FULL_IMAGE: &[u8] = include_bytes!("../resources/lyranis/lyranis-chat2.png");
#[cfg(all(feature = "lyranis",feature = "tui"))]
pub(crate) const LYRANIS_TEXT_IMAGE: &str  = include_str!("../resources/lyranis/lyranis2.txt");

// Commoner Includes:

pub(crate) const COMMONER_INIT_PROMPT:   &str    = include_str!("../resources/commoner_init_prompt.tpl");
pub(crate) const COMMONER_SYSTEM_PROMPT: &str    = include_str!("../resources/commoner_system_prompt.tpl");
