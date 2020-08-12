use clap::Clap;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Clap)]
#[clap(version = clap::crate_version!(), author = "Alistair A. Israel <aisrael@gmail.com>")]
struct Opts {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    #[clap(long)]
    cacert: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // println!(concat!("doggo ", clap::crate_version!()));
    let opts: Opts = Opts::parse();

    let mut builder = reqwest::blocking::Client::builder();
    if let Some(cacert) = opts.cacert {
        println!("cacert: {}", cacert);

        let mut buf = Vec::new();
        File::open(&cacert)?.read_to_end(&mut buf)?;
        println!("vec: {}", buf.len());
        if let Ok(cert) = reqwest::Certificate::from_pem(&buf) {
            builder = builder.add_root_certificate(cert);
        } else if let Ok(cert) = reqwest::Certificate::from_der(&buf) {
            builder = builder.add_root_certificate(cert);
        } else {
            println!("Error reading certificate {}!", cacert);
            std::process::exit(1);
        }
    }

    let dd_api_key = std::env::var("DD_API_KEY")?;

    let client = &builder.use_rustls_tls().build()?;
    let resp = client
        .get("https://api.datadoghq.com/api/v1/validate")
        .header("DD-API-KEY", dd_api_key)
        .send()?
        .json::<HashMap<String, serde_json::value::Value>>()?;
    println!("{:#?}", resp);
    Ok(())
}
