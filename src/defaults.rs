use env_logger::TimestampPrecision;
use log::LevelFilter;

use crate::logger::LogTarget;

// Build Constants:
pub(crate) const APP_NAME:   &str = "Gromrik";
pub(crate) const BUILD_NAME: &str = env!("BUILD_NAME");
pub(crate) const VERSION_ID: &str = env!("CARGO_PKG_VERSION");
pub(crate) const BUILD_TIME: &str = env!("BUILD_TIME");
pub(crate) const BUILD_ID:   &str = env!("BUILD_ID");
pub(crate) const AUTHORS:    &str = env!("CARGO_PKG_AUTHORS");
pub(crate) const COPYRIGHT:  &str = "2026";
pub(crate) const LICENSE:    &str = env!("CARGO_PKG_LICENSE");

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
pub(crate) const DEFAULT_CONFIG_FILE: &str = "config/gromrik.cfg";

// Gromrik Constants:
pub(crate) const DEFAULT_LLM_SERVER_URL: &str = "http://localhost:11434/api/chat";
pub(crate) const DEFAULT_MODEL:          &str = "qwen2.5:7b";
pub(crate) const GROMRIK_EMOJI:          &str = "\u{1F624}\u{26Cf}\u{FE0F}";
pub(crate) const HUMAN_EMOJI:            &str = "\u{1F914}\u{1F4AC}";

// Includes:
pub(crate) const SYSTEM_PROMPT_TEMPLATE: &str = include_str!("../resources/system_prompt.tpl");
