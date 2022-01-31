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
        let mut categories = Vec::new();

        for object in ontology.graph {
            if let serde_json::Value::Object(object_value) = object {
                if let Some(v) = object_value.get("rdf:type") {
                    if let serde_json::Value::Object(object_type) = v {
                        let id = object_value
                            .get("@id")
                            .unwrap()
                            .as_str()
                            .unwrap_or_default()
                            .trim_start_matches("sho:");
                        let name = object_value
                            .get("name")
                            .unwrap()
                            .as_str()
                            .unwrap_or_default();
                        let description = object_value
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
                        if object_type.get("@id").map_or(false, |v| v == "sho:Hazard") {
                            hazards.push(id.to_owned());
                        } else if object_type
                            .get("@id")
                            .map_or(false, |v| v == "sho:Category")
                        {
                            categories.push(id.to_owned());
                        }
                    }
                }
            }
        }

        context.insert("hazards".to_string(), Value::from_serializable(&hazards));
        context.insert(
            "categories".to_string(),
            Value::from_serializable(&categories),
        );

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
