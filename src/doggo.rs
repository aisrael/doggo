use anyhow::Result;
use reqwest::blocking::Response;
use std::path::PathBuf;

pub mod commands;

/// The Doggo client/request context. Basically built from the CLI arguments and/or
/// environment
#[derive(Debug, Default)]
pub struct Context {
    pub api_key: String,
    pub app_key: Option<String>,
    pub cacert_file: Option<PathBuf>,
}

pub trait Executable {
    fn execute(&self, context: &Context) -> Result<Response>;
}

#[derive(Debug, Default)]
pub struct CommandResult {}
