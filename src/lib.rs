use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

pub mod init;

pub(crate) mod chat;
pub(crate) mod config;
pub(crate) mod defaults;
pub(crate) mod splash;
pub(crate) mod util;
