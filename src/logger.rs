use log::{LevelFilter,Level};
use std::str::FromStr;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Write;
use regex::Regex;
use env_logger::TimestampPrecision;
use std::fs::create_dir_all;
use std::path::PathBuf;
use std::fs::OpenOptions;

use crate::defaults::*;
use crate::splash::*;

///////////// Logger

#[allow(dead_code)]
pub(crate) enum LogTarget {
  Stdout,
  File(&'static str,&'static str),
}

impl LogTarget {
  fn into_env_logger_target(self) -> env_logger::fmt::Target {
    match self {
      LogTarget::Stdout => env_logger::fmt::Target::Stdout,
      LogTarget::File(dir, filename) => {
        if !dir.is_empty() {
          if let Err(err) = create_dir_all(dir) {
            eprintln!("Warning: Failed to create log directory '{}' ({}). Falling back to Stdout.", dir, err);
            return env_logger::fmt::Target::Stdout;
          }
        }
        let mut path = PathBuf::from(dir);
        path.push(filename);
        let file = match OpenOptions::new().create_new(true).write(true).open(&path) {
          Ok(mut new_file) => {
            if let Err(err) = writeln!(new_file,"{}",raw_splash()) {
              eprintln!("Warning: Failed to write header to '{:?}' ({}).", path, err);
            }
            Ok(new_file)
          }
          Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => {
            OpenOptions::new().append(true).open(&path)
          }
          Err(err) => Err(err),
        };
        match file {
          Ok(file) => env_logger::fmt::Target::Pipe(Box::new(file)),
          Err(err) => {
            eprintln!("Warning: Failed to open log file '{:?}' ({}). Falling back to Stdout.", path, err);
            env_logger::fmt::Target::Stdout
          }
        }
      },
    }
  }
}

pub fn init() {
  let mut redact_patterns = Vec::new();
  if let Ok(file) = File::open(REDACT_LOG_LIST) {
    let reader = BufReader::new(file);
    for line in reader.lines().map_while(Result::ok) {
      let trimmed = line.trim();
      if !trimmed.is_empty() && !trimmed.starts_with('#') {
        let escaped = regex::escape(trimmed);
        redact_patterns.push(escaped);
      }
    }
  }

  let filter_regex = if !redact_patterns.is_empty() {
    let unified_pattern = redact_patterns.join("|");
    Regex::new(&format!("({})", unified_pattern)).ok()
  } else {
    None
  };

  let loglevel: LevelFilter = match std::env::var("RUST_LOG") {
    Ok(level) => match Level::from_str(level.as_str()) {
      Ok(level) => level.to_level_filter(),
      Err(_)    => DEFAULT_LOG_LEVEL,
    },
    Err(_)    => DEFAULT_LOG_LEVEL,
  };

  let mut builder = env_logger::Builder::from_default_env();
  builder
    .filter_level(loglevel)
    .target(DEFAULT_LOG_TARGET.into_env_logger_target())
    .format(move |buf, record| {
      let original_msg = format!("{}", record.args());
      let redacted_msg = if let Some(ref regex) = filter_regex {
        regex.replace_all(&original_msg, "[REDACTED]")
      } else {
        std::borrow::Cow::Borrowed(&*original_msg)
      };
      let mut metadata = Vec::new();
      if let Some(precision) = DEFAULT_LOG_FORMAT_TIMESTAMP {
        let ts = match precision {
          TimestampPrecision::Seconds => format!("{}", buf.timestamp_seconds()),
          TimestampPrecision::Millis  => format!("{}", buf.timestamp_millis()),
          TimestampPrecision::Micros  => format!("{}", buf.timestamp_micros()),
          TimestampPrecision::Nanos   => format!("{}", buf.timestamp_nanos()),
        };
        metadata.push(ts);
      }
      if DEFAULT_LOG_FORMAT_LEVEL {
        metadata.push(format!("{}", record.level()));
      }
      if DEFAULT_LOG_FORMAT_TARGET {
        metadata.push(format!("{}", record.target()));
      }
      if metadata.is_empty() {
        writeln!(buf, "[] {}", redacted_msg)
      } else {
        writeln!(buf, "[{}] {}", metadata.join(" "), redacted_msg)
      }
    });
  for &module in DEFAULT_LOG_NO_DEBUG_MODULES {
    builder.filter_module(module,log::LevelFilter::Info);
  }
  builder.init();
}

