use std::fmt;
use std::collections::HashMap;

use crate::config::*;
use crate::defaults::*;

use crate::util;

///////////// Help

#[derive(Debug,Clone)]
pub(crate) struct Help {
  config: Config,
}

impl fmt::Display for Help {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let details: Vec<String> = vec![
      format!("  --config <CONFIG FILE PATH>          Path to the config file (default: {})",DEFAULT_CONFIG_FILE),
      format!("  --persona <PERSONA>                  Persona to interact with (default: {}, choices: {}",DEFAULT_PERSONA,PERSONA_LIST),
      format!("  --llm-server-url <LLM SERVER URL>    URL for LLM chat endpoint, must be compatible with Ollama's /api/chat (default: {})",DEFAULT_LLM_SERVER_URL),
      format!("  --model <LLM MODEL NAME>             LLM model to use for interacting with the persona, must be already loaded into the server (default: {})",DEFAULT_MODEL),
      format!("  --persona-file <PERSONA BUNDLE PATH> Path to a persona bundle (.grom file) to use instead of the preloaded personas. (default: none)"),
      #[cfg(any(feature = "tui",feature = "gui",feature = "web"))]
      format!("  --history-file <HISTORY FILE PATH>   Path to the default file to use for saving and loading history (default: {})",DEFAULT_HISTORY_FILE),
      format!("  --mode <MODE>                        Mode to run in, (default: {}, choices: {}",DEFAULT_MODE,MODE_LIST.join(", ")),
      #[cfg(any(feature = "cli"))]
      format!("  --cli                                Run in command line mode"),
      #[cfg(any(feature = "tui"))]
      format!("  --tui                                Run in TUI mode"),
      #[cfg(any(feature = "gui"))]
      format!("  --gui                                Run in GUI mode"),
      #[cfg(any(feature = "web"))]
      format!("  --web                                Run in web server mode"),
      format!("  --help                               Display this help message"),
    ];
    let fields: HashMap<&str,String> = HashMap::from([
      ("RUNAS",self.config.ranas.clone()),
      ("DESCRIPTION",DESCRIPTION.to_string()),
      ("DETAILS",details.join("\n")),
      ("COPYRIGHT",COPYRIGHT.to_string()),
      ("AUTHORS",AUTHORS.to_string()),
    ]);
    let help: String = util::render(&HELP_TEMPLATE.to_string(),&fields);
    write!(f, "{}",help)
  }
}

impl Help {
  pub(crate) fn new(config: &Config) -> Help {
    Help { config: config.clone() }
  }
}

