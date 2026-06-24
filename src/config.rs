use std::fmt;
use std::fs;
use std::fs::{read_to_string,write};
use std::env::{args,Args};

use crate::defaults::*;
use crate::util;

#[derive(Debug, Clone)]
pub(crate) struct Config {
  pub(crate) llm_server_url: String,
  pub(crate) model:          String,
}

impl fmt::Display for Config {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f,
"
  -----------------------------------
             Configuration 
  -----------------------------------

    LLM Server URL: {}
    Model:          {}

  -----------------------------------
",
      self.llm_server_url,
      self.model,
    )
  }
}

impl Config {
  pub(crate) fn defaults() -> Config {
    Config {
      llm_server_url: DEFAULT_LLM_SERVER_URL.to_string(),
      model:          DEFAULT_MODEL.to_string(),
    }
  }
  
  pub fn write(&self,config_file: &String) -> Result<String,String> {
    let config_file: String = util::build_path(&config_file,&"config".to_string());
    let mut config: Vec<String> = Vec::new();
    config.push(format!("llm_server_url: {}",self.llm_server_url));
    config.push(format!("model: {}",self.model));
    match write(&config_file,config.join("\n")) {
      Ok(()) => Ok(format!("Outputted config to {}",config_file)),
      Err(err) => Err(format!("Failed to output config to {}: {}",config_file,err.to_string()))
    }
  }

  pub fn new() -> Config {
    let mut config = Config::defaults();
    config.llm_server_url = util::prompt(format!("LLM Server URL: (default: {})",config.llm_server_url),config.llm_server_url.clone());
    config.model          = util::prompt(format!("Model: (default: {})",config.model),config.model.clone());
    println!("\n");
    config
  }

  pub fn from_file(config: Option<Config>,config_path: &String) -> Config {
    let mut config_file: String = Config::path_from_args(config_path);
    config_file = match fs::exists(&config_file) {
      Ok(true) => config_file.clone(),
      Ok(false) => {
        println!("Config file {} does not exist.",config_file);
        let config: Config = Config::new();
        let to_file: bool = util::prompt(format!("Write config to new file at {}? (true/false, default false)",config_file),"false".to_string()).parse().unwrap_or(false);
        if to_file {
          match config.write(&config_file) {
            Ok(message) => println!("{}",message),
            Err(err) => eprintln!("{}",err),
          }
        }
        return config
      },
      Err(err) => {
        eprintln!("Error testing if {} exists: {}",config_file,err);
        return Config::defaults()
      }
    };
    println!("Loading config from {}.",config_file);
    let lines: Vec<String> = match read_to_string(&config_file) {
      Ok(lines) => lines,
      Err(err)  => {
        eprintln!("Failed to read from config file, using defaults: {}",err);
        String::new()
      },
    }.lines().map(|line| line.trim()).filter(|line| !line.is_empty()).filter(|line| !line.starts_with("#")).map(String::from).collect();
    let mut config = if let Some(config) = config { config } else { Config::defaults() };
    for line in lines {
      let pair: Vec<&str> = line.split(":").collect();
      if pair.len() > 1 {
        let value: String = pair[1..].join(":").trim_start().to_string();
        match pair[0] {
          "llm_server_url" => config.llm_server_url = value.clone(),
          "model"          => config.model = value.clone(),
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
    let config_file: String = DEFAULT_CONFIG_FILE.to_string();
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
        "--llm-server-url" => match args.next() {
          Some(value) => config.llm_server_url = value.trim().to_string(),
          None => {},
        },
        "--model" => match args.next() {
          Some(value) => config.model = value.trim().to_string(),
          None => {},
        },
        // Error on everything else:
        option => if option.starts_with("--") { eprintln!("{}: unrecognized option -- '{}'",util::bin_name(),option);
        },
      }
    }
    config
  }
}
