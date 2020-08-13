use crate::doggo::{Context, Executable};
use anyhow::Result;
use clap::Clap;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn reqwest_client_builder_from_context(
    context: &Context,
) -> Result<reqwest::blocking::ClientBuilder> {
    let mut builder = reqwest::blocking::Client::builder();
    if let Some(cacert_file) = &context.cacert_file {
        let cacert_str = cacert_file.to_string_lossy();
        println!("cacert: {}", cacert_str);

        let mut buf = Vec::new();
        File::open(&cacert_file)?.read_to_end(&mut buf)?;
        println!("vec: {}", buf.len());
        if let Ok(cert) = reqwest::Certificate::from_pem(&buf) {
            builder = builder.add_root_certificate(cert);
        } else if let Ok(cert) = reqwest::Certificate::from_der(&buf) {
            builder = builder.add_root_certificate(cert);
        } else {
            println!("Error reading certificate {}!", cacert_str);
            std::process::exit(1);
        }
    }
    Ok(builder.use_rustls_tls())
}

/// Verify connectivity by attempting to authenticate with the Datadog HTTP API
#[derive(Clap, Default)]
pub struct Authenticate {}

impl Executable for Authenticate {
    fn execute(&self, context: &Context) -> Result<String> {
        let builder = reqwest_client_builder_from_context(context)?;
        let client = &builder.build()?;
        let resp = client
            .get("https://api.datadoghq.com/api/v1/validate")
            .header("DD-API-KEY", &context.api_key)
            .send()?
            .json::<HashMap<String, serde_json::value::Value>>()?;
        println!("{:#?}", resp);
        Ok(String::from(""))
    }
}
