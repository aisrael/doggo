use std::path::PathBuf;

pub mod commands;

/// The Doggo client/request context. Basically built from the CLI arguments and/or
/// environment
#[derive(Debug, Default)]
pub struct Context {
    pub api_key: String,
    pub cacert_file: Option<PathBuf>,
}
