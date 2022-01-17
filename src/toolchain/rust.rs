use std::collections::HashMap;
use std::path::{Path, PathBuf};

use minijinja::value::Value;

use crate::{builtin_templates, BuildTemplate, Ontology};

static RUST_TEMPLATES: &[(&str, &str)] = &builtin_templates!["rust" =>
    ("rs.api", "api.rs")
];

pub(crate) struct Rust;

impl Rust {
    pub(crate) fn create() -> Self {
        Self
    }

    fn project_structure(output_path: &Path) -> HashMap<PathBuf, &'static str> {
        let output = output_path.to_path_buf();

        let mut template_files = HashMap::new();

        template_files.insert(output.join("sifis-rust-api.rs"), "rs.api");

        template_files
    }
}

impl BuildTemplate for Rust {
    fn define(
        &self,
        ontology: Ontology,
        output_path: &Path,
    ) -> (HashMap<PathBuf, &'static str>, HashMap<String, Value>) {
        let mut context = HashMap::new();

        let check_type =
            |v: &serde_json::Value| v.as_str().unwrap_or_default() == "owl:NamedIndividual";
        for object in ontology.graph {
            if let serde_json::Value::Object(hazard) = object {
                if hazard.get("@type").map_or(false, |v| check_type(v)) {
                    let id = hazard
                        .get("@id")
                        .unwrap()
                        .as_str()
                        .unwrap_or_default()
                        .replace(":", "_");
                    let name = hazard.get("name").unwrap().as_str().unwrap_or_default();
                    let description = hazard
                        .get("description")
                        .unwrap()
                        .as_str()
                        .unwrap_or_default();
                    context.insert(
                        id.to_owned() + "_name",
                        Value::from_serializable(&name.to_owned()),
                    );
                    context.insert(
                        id.to_owned() + "_description",
                        Value::from_serializable(&description.to_owned()),
                    );
                }
            }
        }

        let files = Rust::project_structure(output_path);

        (files, context)
    }

    fn get_templates() -> &'static [(&'static str, &'static str)] {
        RUST_TEMPLATES
    }
}
