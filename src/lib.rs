#[cfg(not(any(feature = "cli",feature = "tui",feature = "gui",feature = "web")))]
compile_error!("At least one user interface feature is required to be enabled: 'cli', 'tui', 'gui', or 'web'.");

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

pub mod init;
pub mod logger;

pub(crate) mod chat;
pub(crate) mod cli;
pub(crate) mod config;
pub(crate) mod defaults;
pub(crate) mod gui;
pub(crate) mod mode;
pub(crate) mod persona;
pub(crate) mod splash;
pub(crate) mod tui;
pub(crate) mod ui;
pub(crate) mod util;
pub(crate) mod web;
