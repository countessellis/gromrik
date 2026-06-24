// Build Constants:
pub(crate) const APP_NAME:   &str = "Gromrik";
pub(crate) const BUILD_NAME: &str = env!("BUILD_NAME");
pub(crate) const VERSION_ID: &str = env!("CARGO_PKG_VERSION");
pub(crate) const BUILD_TIME: &str = env!("BUILD_TIME");
pub(crate) const BUILD_ID:   &str = env!("BUILD_ID");
pub(crate) const AUTHORS:    &str = env!("CARGO_PKG_AUTHORS");
pub(crate) const COPYRIGHT:  &str = "2026";
pub(crate) const LICENSE:    &str = env!("CARGO_PKG_LICENSE");

// Paths:
pub(crate) const DEFAULT_CONFIG_FILE: &str = "config/gromrik.cfg";

// Gromrik:
pub(crate) const DEFAULT_LLM_SERVER_URL: &str = "http://localhost:11434/api/chat";
pub(crate) const DEFAULT_MODEL:          &str = "mistral:7b-instruct";

// Includes:
pub(crate) const SYSTEM_PROMPT_TEMPLATE: &str = include_str!("../resources/system_prompt.tpl");
