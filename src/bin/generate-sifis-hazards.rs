use std::path::PathBuf;

use clap::builder::{PossibleValuesParser, TypedValueParser};
use clap::Parser;

use tracing_subscriber::EnvFilter;

use generate_sifis_hazards::{HazardsProducer, Templates};

#[derive(Parser, Debug)]
struct Opts {
    /// Path to the ontology file
    #[clap(short = 'p', value_hint = clap::ValueHint::DirPath)]
    ontology_path: PathBuf,
    /// Path to the generated API
    #[clap(short, value_hint = clap::ValueHint::DirPath)]
    output_path: PathBuf,
    /// Name of a builtin template
    #[clap(long, short, value_parser = PossibleValuesParser::new(Templates::all())
        .map(|s| s.parse::<Templates>().unwrap()))]
    template: Templates,
    /// Output the generated paths as they are produced
    #[clap(short, long)]
    verbose: bool,
}

fn main() {
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

    HazardsProducer::new()
        .run(opts.template, opts.ontology_path, opts.output_path)
        .unwrap();
}
