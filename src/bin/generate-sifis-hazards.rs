use std::path::PathBuf;

use structopt::clap::AppSettings;
use structopt::StructOpt;

use generate_sifis_hazards::{adds_hazards_to_api, Templates};

#[derive(StructOpt, Debug)]
#[structopt(global_settings=&[AppSettings::ColoredHelp])]
struct Opts {
    /// Name of a builtin template
    #[structopt(long, short, possible_values = &Templates::variants())]
    template: Templates,
    /// Path to the ontology file
    #[structopt(parse(from_os_str))]
    ontology_path: PathBuf,
    /// Path to the generated API
    #[structopt(parse(from_os_str))]
    output_path: PathBuf,
}

lazy_static::lazy_static! {
    static ref TEMPLATES_INFO: String = Templates::info();
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::from_args();

    adds_hazards_to_api(opts.template, &opts.ontology_path, &opts.output_path)?;

    Ok(())
}
