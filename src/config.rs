use std::fmt;
use std::fs;
use std::fs::{read_to_string,write};
use std::env::{args,Args};
use std::str::FromStr;

use crate::defaults::*;
use crate::mode::*;
use crate::persona::*;
use crate::util;

#[derive(Debug, Clone)]
pub(crate) struct Config {
  pub(crate) ranas:          String,
  pub(crate) mode:           Mode,
  pub(crate) persona:        Persona,
  pub(crate) llm_server_url: String,
  pub(crate) model:          String,

  #[cfg(any(feature = "tui",feature = "gui",feature = "web"))]
  pub(crate) history_file:   String,
}

impl fmt::Display for Config {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let history = {
      #[cfg(any(feature = "tui", feature = "gui", feature = "web"))]
      {
        &self.history_file
      }
      #[cfg(not(any(feature = "tui", feature = "gui", feature = "web")))]
      {
        "N/A"
      }
    };
    write!(f,
"
  -----------------------------------
             Configuration 
  -----------------------------------

    Mode:           {}
    Persona:        {}
    LLM Server URL: {}
    Model:          {}
    History File:   {}

  -----------------------------------
",
      self.mode,
      self.persona.name,
      self.llm_server_url,
      self.model,
      history,
    )
  }
}

impl Config {
  pub(crate) fn defaults() -> Config {
    Config {
      ranas:          util::bin_name(),
      mode:           Mode::mode(),
      persona:        Persona::new(DEFAULT_PERSONA),
      llm_server_url: DEFAULT_LLM_SERVER_URL.to_string(),
      model:          DEFAULT_MODEL.to_string(),
      #[cfg(any(feature = "tui",feature = "gui",feature = "web"))]
      history_file:   DEFAULT_HISTORY_FILE.to_string(),
    }
  }
  
  pub fn write(&self,config_file: &String) -> Result<String,String> {
    let config_file: String = util::build_path(&config_file,&"config".to_string());
    let mut config: Vec<String> = Vec::new();
    config.push(format!("mode: {}",self.mode));
    config.push(format!("persona: {}",self.persona.name.to_lowercase()));
    config.push(format!("llm_server_url: {}",self.llm_server_url));
    config.push(format!("model: {}",self.model));
    #[cfg(any(feature = "tui",feature = "gui",feature = "web"))]
    config.push(format!("history_file: {}",self.history_file));
    match write(&config_file,config.join("\n")) {
      Ok(()) => Ok(format!("Outputted config to {}",config_file)),
      Err(err) => Err(format!("Failed to output config to {}: {}",config_file,err.to_string()))
    }
  }

  pub fn new() -> Config {
    let mut config = Config::defaults();
    config.mode = match Mode::from_str(util::prompt(format!("Mode: (cli/tui/gui/web, default: {})",config.mode),config.mode.to_string()).as_str()) {
      Ok(mode) => mode,
      Err(_)   => Default::default(),
    };
    config.persona        = Persona::new(util::prompt(format!("Persona: ({}, default: {})",PERSONA_LIST,config.persona.name),config.persona.name).as_str());
    config.llm_server_url = util::prompt(format!("LLM Server URL: (default: {})",config.llm_server_url),config.llm_server_url.clone());
    config.model          = util::prompt(format!("Model: (default: {})",config.model),config.model.clone());
    #[cfg(any(feature = "tui",feature = "gui",feature = "web"))]
    {
      config.history_file   = util::prompt(format!("History Save File: (default: {})",config.history_file),config.history_file.clone());
    }
    println!("\n");
    config
  }

  pub fn from_file(config: Option<Config>,config_file: &String) -> Config {
    let config_file: String = match fs::exists(&config_file) {
      Ok(true) => config_file.clone(),
      Ok(false) => {
        println!("Config file {} does not exist.",config_file);
        let config: Config = Config::new();
        let to_file: bool = util::prompt(format!("Write config to new file at {}? (true/false, default false)",config_file),"false".to_string()).parse().unwrap_or(false);
        if to_file {
          match config.write(&config_file) {
            Ok(message) => log::info!("{}",message),
            Err(err) => log::error!("{}",err),
          }
        }
        return config
      },
      Err(err) => {
        log::error!("Error testing if {} exists: {}",config_file,err);
        return Config::defaults()
      }
    };
    log::info!("Loading config from {}.",config_file);
    println!("Loading config from {}.\n",config_file);
    let lines: Vec<String> = match read_to_string(&config_file) {
      Ok(lines) => lines,
      Err(err)  => {
        log::error!("Failed to read from config file, using defaults: {}",err);
        String::new()
      },
    }.lines().map(|line| line.trim()).filter(|line| !line.is_empty()).filter(|line| !line.starts_with("#")).map(String::from).collect();
    let mut config = if let Some(config) = config { config } else { Config::defaults() };
    for line in lines {
      let pair: Vec<&str> = line.split(":").collect();
      if pair.len() > 1 {
        let value: String = pair[1..].join(":").trim_start().to_string();
        match pair[0] {
          "mode"     => config.mode = match Mode::from_str(value.as_str()) {
            Ok(mode) => mode,
            Err(_)   => Default::default(),
          },
          "persona"        => config.persona = Persona::new(value.as_str()),
          "llm_server_url" => config.llm_server_url = value.clone(),
          "model"          => config.model = value.clone(),
          #[cfg(any(feature = "tui",feature = "gui",feature = "web"))]
          "history_file"   => config.history_file = value.clone(),
          // Ignore everything else:
          _ => {},
        };
      }
    }
    Config::from_args(&config)
  }

  pub fn path_from_args(default_path: &String) -> String {
    let mut args: Args = args();
    while let Some(arg) = args.next() {
      match arg.as_str().trim() {
        "--config" => {
          match args.next() {
            Some(config_file) => return config_file.trim().to_string(),
            None              => {},
          }
        },
        _ => {},
      }
    }
    default_path.clone()
  }

  pub fn get_config() -> Config {
    let config_file: String = Self::path_from_args(&DEFAULT_CONFIG_FILE.to_string());
    Config::from_file(None,&config_file)
  }

  pub fn from_args(config: &Config) -> Config {
    let mut args: Args = args();
    let mut config: Config = config.clone();
    while let Some(arg) = args.next() {
      match arg.as_str().trim() {
        // Ignore flags processed elsewhere:
        "--config" => {},
        // Process options:
        "--persona" => match args.next() {
          Some(value) => config.persona = Persona::new(value.trim()),
          None => {},
        },
        "--llm-server-url" => match args.next() {
          Some(value) => config.llm_server_url = value.trim().to_string(),
          None => {},
        },
        "--model" => match args.next() {
          Some(value) => config.model = value.trim().to_string(),
          None => {},
        },
        #[cfg(any(feature = "tui",feature = "gui",feature = "web"))]
        "--history-file" => match args.next() {
          Some(value) => config.history_file = value.trim().to_string(),
          None => {},
        },
        "--mode" => match args.next() {
          Some(mode) => match Mode::from_str(mode.as_str()) {
            Ok(mode) => config.mode = mode,
            Err(_)   => {},
          },
          None        => {},
        },
        "--cli" => config.mode = Mode::CLI,
        "--tui" => config.mode = Mode::TUI,
        "--gui" => config.mode = Mode::GUI,
        "--web" => config.mode = Mode::WEB,
        "--help" => config.mode = Mode::Help,
        // Error on everything else:
        option => if option.starts_with("--") { log::error!("{}: unrecognized option -- '{}'",util::bin_name(),option);
        },
      }
    }
    config
  }
}
