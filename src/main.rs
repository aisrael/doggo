mod doggo;

use clap::Clap;
use doggo::{Context, Executable};
use std::path::PathBuf;

/// A binary Datadog API client
#[derive(Clap)]
#[clap(version = clap::crate_version!(), author = "Alistair A. Israel")]
struct Opts {
    /// CA certificate to verify peer against
    #[clap(long, parse(from_os_str))]
    cacert: Option<PathBuf>,

    /// your Datadog API key. You can also set the environment variables DATADOG_API_KEY or DD_API_KEY
    #[clap(long)]
    api_key: Option<String>,

    /// your Datadog application key. You can also set the environment variables DATADOG_APP_KEY or DD_APP_KEY
    #[clap(long)]
    app_key: Option<String>,

    /// verbose
    #[clap(short, long)]
    verbose: bool,

    /// quiet (suppress output)
    #[clap(short, long)]
    quiet: bool,

    /// the command to execute
    #[clap(subcommand)]
    command: Command,
}

/// The command to execute
#[derive(Clap)]
enum Command {
    #[clap()]
    Authenticate,
    #[clap()]
    Metric(Metric),
}

/// Post metrics
#[derive(Clap)]
struct Metric {
    #[clap(subcommand)]
    subcommand: MetricSubcommand,
}

/// The metric subcommand
#[derive(Clap)]
enum MetricSubcommand {
    #[clap()]
    Post(Post),
}

/// Post metrics
#[derive(Clap)]
struct Post {
    /// scopes your metric to a specific host (default to the local host name)
    #[clap(long)]
    host: Option<String>,
    /// type of the metric - gauge(32bit float) or counter(64bit integer)
    #[clap(name = "type", long)]
    metric_type: String,
    /// metric name
    #[clap()]
    name: String,
    /// metric value
    #[clap()]
    value: String,
    /// comma-separated list of tags
    #[clap(short, long)]
    tags: Vec<String>,
}

fn build_context_from_opts(opts: &Opts) -> Context {
    let api_key = if let Some(api_key) = &opts.api_key {
        api_key.clone()
    } else if let Ok(api_key) = std::env::var("DATADOG_API_KEY") {
        api_key
    } else if let Ok(api_key) = std::env::var("DD_API_KEY") {
        api_key
    } else {
        println!("No --api-key provided and neither $DATADOG_API_KEY nor $DD_API_KEY are set!");
        std::process::exit(1);
    };

    let app_key = if let Some(app_key) = &opts.app_key {
        Some(app_key.clone())
    } else if let Ok(app_key) = std::env::var("DATADOG_APP_KEY") {
        Some(app_key)
    } else if let Ok(app_key) = std::env::var("DD_APP_KEY") {
        Some(app_key)
    } else {
        match &opts.command {
            Command::Metric(_) => {
                println!(
                    "No --app-key provided and neither $DATADOG_APP_KEY nor $DD_APP_KEY are set!"
                );
                std::process::exit(1);
            }
            _ => None,
        }
    };

    Context {
        api_key: api_key,
        app_key: app_key,
        cacert_file: opts.cacert.clone(),
        verbose: opts.verbose,
    }
}

fn executable_from_opts(opts: Opts) -> Box<dyn Executable> {
    match opts.command {
        Command::Authenticate => Box::new(doggo::commands::Authenticate::default()),
        Command::Metric(metric) => match metric.subcommand {
            MetricSubcommand::Post(post) => Box::new(doggo::commands::PostMetric {
                host: post.host,
                metric_type: post.metric_type,
                metric_name: post.name,
                value: post.value,
                tags: Some(post.tags),
            }),
        },
    }
}

fn main() {
    let opts: Opts = Opts::parse();
    let context = build_context_from_opts(&opts);
    let quiet = opts.quiet;
    let command = executable_from_opts(opts);

    match command.execute(&context) {
        Ok(resp) => {
            let status = resp.status();
            if !quiet {
                println!("{}", resp.text().unwrap());
            }
            if status.is_client_error() || status.is_server_error() {
                std::process::exit(1);
            }
        }
        Err(e) => {
            if !quiet {
                println!("{}", e);
            }
            std::process::exit(1);
        }
    }
}
