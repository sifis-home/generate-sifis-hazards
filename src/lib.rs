mod filters;
mod toolchain;

use std::collections::HashMap;
use std::fs::{create_dir_all, write, File};
use std::io::BufReader;
use std::path::{Path, PathBuf};

use anyhow::Result;
use arg_enum_proc_macro::ArgEnum;
use minijinja::value::Value;
use minijinja::{Environment, Source};
use serde::Deserialize;
use tracing::debug;

use filters::*;
use toolchain::*;

/// Supported templates
#[derive(ArgEnum, Debug)]
pub enum Templates {
    /// Generate hazards documentation for Rust APIs
    #[arg_enum(name = "rust")]
    Rust,
}

impl Templates {
    pub fn info() -> String {
        let mut info = "Available built-in templates:\n".to_string();
        for (names, description) in Templates::descriptions() {
            std::fmt::write(
                &mut info,
                format_args!("    {:<15} {}\n", names[0], description[0]),
            )
            .unwrap();
        }

        info
    }
}

#[derive(Deserialize)]
struct Ontology {
    #[serde(rename = "@graph")]
    graph: Vec<serde_json::Value>,
    #[serde(skip_deserializing)]
    _context: serde_json::Value,
}

struct SifisTemplate {
    context: HashMap<String, Value>,
    files: HashMap<PathBuf, &'static str>,
    dirs: Vec<PathBuf>,
    source: Source,
}

impl SifisTemplate {
    fn render(self) -> Result<()> {
        let mut env = Environment::new();
        let SifisTemplate {
            context,
            files,
            dirs,
            source,
        } = self;

        // Create dirs
        for dir in dirs {
            debug!("Creating {}", dir.display());
            create_dir_all(dir)?
        }

        env.set_source(source);
        env.add_filter("hypens_to_underscores", hypens_to_underscores);

        // Fill in templates
        for (path, template_name) in files {
            debug!("Creating {}", path.display());
            let template = env.get_template(template_name)?;
            let filled_template = template.render(&context)?;
            write(path, filled_template)?;
        }

        Ok(())
    }
}

/// Build a template
trait BuildTemplate {
    fn define(
        &self,
        ontology: Ontology,
        output_path: &Path,
    ) -> (
        HashMap<PathBuf, &'static str>,
        Vec<PathBuf>,
        HashMap<String, Value>,
    );

    fn get_templates() -> &'static [(&'static str, &'static str)];

    fn build(&self, ontology: Ontology, output_path: &Path) -> SifisTemplate {
        let (files, dirs, context) = self.define(ontology, output_path);
        let source = build_source(Self::get_templates());

        SifisTemplate {
            context,
            files,
            dirs,
            source,
        }
    }
}

fn build_source(templates: &[(&str, &str)]) -> Source {
    let mut source = Source::new();
    for (name, src) in templates {
        source
            .add_template(*name, *src)
            .expect("Internal error, built-in template");
    }

    source
}

/// Adds hazards to Sifis APIs
pub fn adds_hazards_to_api(
    template_type: Templates,
    ontology_path: &Path,
    output_path: &Path,
) -> Result<()> {
    // Deserialize ontology
    let file = File::open(ontology_path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let ontology = serde_json::from_reader(reader)?;

    let template = match template_type {
        Templates::Rust => Rust::create().build(ontology, output_path),
    };

    template.render()
}
