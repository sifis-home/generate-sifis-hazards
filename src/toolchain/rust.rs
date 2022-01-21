use std::collections::HashMap;
use std::path::{Path, PathBuf};

use minijinja::value::Value;

use crate::{builtin_templates, BuildTemplate, Ontology};

static RUST_TEMPLATES: &[(&str, &str)] = &builtin_templates!["rust" =>
    ("rs.api", "api.rs"),
    ("toml.Cargo", "Cargo.toml")
];

pub(crate) struct Rust;

impl Rust {
    pub(crate) fn create() -> Self {
        Self
    }

    fn project_structure(
        api_name: &str,
        output_path: &Path,
    ) -> (HashMap<PathBuf, &'static str>, Vec<PathBuf>) {
        let output = output_path.to_path_buf().join(api_name);
        let src = output.join("src");

        let mut template_files = HashMap::new();

        template_files.insert(output.join("Cargo.toml"), "toml.Cargo");
        template_files.insert(src.join("lib.rs"), "rs.api");

        (template_files, vec![output, src])
    }
}

impl BuildTemplate for Rust {
    fn define(
        &self,
        api_name: &str,
        ontology: Ontology,
        output_path: &Path,
    ) -> (
        HashMap<PathBuf, &'static str>,
        Vec<PathBuf>,
        HashMap<String, Value>,
    ) {
        let mut context = HashMap::new();
        let mut hazards = Vec::new();

        for object in ontology.graph {
            if let serde_json::Value::Object(hazard) = object {
                if let Some(v) = hazard.get("rdf:type") {
                    if let serde_json::Value::Object(hazard_type) = v {
                        if hazard_type.get("@id").map_or(false, |v| v == "sho:Hazard") {
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
                            hazards.push(id.trim_start_matches("sho_").to_owned());
                        }
                    }
                }
            }
        }

        context.insert("hazards".to_string(), Value::from_serializable(&hazards));

        context.insert(
            "name".to_owned(),
            Value::from_serializable(&api_name.to_owned()),
        );

        let (files, dirs) = Rust::project_structure(api_name, output_path);

        (files, dirs, context)
    }

    fn get_templates() -> &'static [(&'static str, &'static str)] {
        RUST_TEMPLATES
    }
}
