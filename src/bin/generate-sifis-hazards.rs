use std::path::PathBuf;

use clap::Parser;
use tracing_subscriber::EnvFilter;

use generate_sifis_hazards::{adds_hazards_to_api, Templates};

#[derive(Parser, Debug)]
struct Opts {
    /// Output the generated paths as they are produced
    #[clap(short, long, global = true)]
    verbose: bool,
    /// Name of a builtin template
    #[clap(long, short, possible_values = Templates::variants())]
    template: Templates,
    /// Path to the ontology file
    #[clap(parse(from_os_str))]
    ontology_path: PathBuf,
    /// Path to the generated API
    #[clap(parse(from_os_str))]
    output_path: PathBuf,
}

lazy_static::lazy_static! {
    static ref TEMPLATES_INFO: String = Templates::info();
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();

    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| {
            if opts.verbose {
                EnvFilter::try_new("debug")
            } else {
                EnvFilter::try_new("info")
            }
        })
        .unwrap();

    tracing_subscriber::fmt()
        .without_time()
        .with_env_filter(filter_layer)
        .with_writer(std::io::stderr)
        .init();

    adds_hazards_to_api(opts.template, &opts.ontology_path, &opts.output_path)?;

    Ok(())
}
