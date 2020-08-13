mod doggo;

use clap::Clap;
use doggo::Context;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

/// A binary Datadog API client
#[derive(Clap)]
#[clap(version = clap::crate_version!(), author = "Alistair A. Israel")]
struct Opts {
    /// CA certificate to verify peer against
    #[clap(long, parse(from_os_str))]
    cacert: Option<PathBuf>,

    /// your API key, from https://app.datadoghq.com/account/settings#api.
    /// You can also set the environment variables DATADOG_API_KEY or DD_API_KEY
    #[clap(short, long)]
    api_key: Option<String>,
}

fn build_context_from_opts() -> Context {
    let opts: Opts = Opts::parse();

    let context = if let Some(api_key) = opts.api_key {
        Context {
            api_key: api_key,
            ..Default::default()
        }
    } else if let Ok(api_key) = std::env::var("DATADOG_API_KEY") {
        Context {
            api_key: api_key,
            ..Default::default()
        }
    } else if let Ok(api_key) = std::env::var("DD_API_KEY") {
        Context {
            api_key: api_key,
            ..Default::default()
        }
    } else {
        println!("No --api-key provided and neither $DATADOG_API_KEY nor $DD_API_KEY are set!");
        std::process::exit(1);
    };

    Context {
        cacert_file: opts.cacert,
        ..context
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let context = build_context_from_opts();

    let mut builder = reqwest::blocking::Client::builder();
    if let Some(cacert_file) = context.cacert_file {
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

    let client = &builder.use_rustls_tls().build()?;
    let resp = client
        .get("https://api.datadoghq.com/api/v1/validate")
        .header("DD-API-KEY", context.api_key)
        .send()?
        .json::<HashMap<String, serde_json::value::Value>>()?;
    println!("{:#?}", resp);
    Ok(())
}
