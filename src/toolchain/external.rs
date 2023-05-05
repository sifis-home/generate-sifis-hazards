use std::collections::HashMap;
use std::path::{Path, PathBuf};

use minijinja::value::Value;
use minijinja::Source;
use serde::Serialize;

use crate::{BuildTemplate, Ontology};

pub(crate) struct External<'a> {
    template: (&'static str, &'a str),
}

impl<'a> External<'a> {
    pub(crate) fn create(template: (&'static str, &'a str)) -> Self {
        Self { template }
    }

    fn project_structure(
        &self,
        output_path: &Path,
    ) -> (HashMap<PathBuf, &'static str>, Vec<PathBuf>) {
        let output = output_path.to_path_buf().join("src");

        let mut template_files = HashMap::new();

        template_files.insert(output.join("ontology.rs"), self.template.0);

        (template_files, vec![output])
    }
}

#[derive(Serialize)]
struct HazardData {
    description: String,
    name: String,
    category: String,
}

#[derive(Serialize)]
struct CategoryData {
    description: String,
    name: String,
    hazards: Vec<String>,
}

impl<'a> BuildTemplate for External<'a> {
    fn define(
        &self,
        ontology: Ontology,
        output_path: &Path,
    ) -> (
        HashMap<PathBuf, &'static str>,
        Vec<PathBuf>,
        HashMap<String, Value>,
    ) {
        let mut context = HashMap::new();
        let mut hazards = Vec::new();
        let mut categories_hazards = HashMap::new();
        let mut categories = Vec::new();

        for object in ontology.graph {
            if let serde_json::Value::Object(object_value) = object {
                if let Some(serde_json::Value::Object(object_type)) = object_value.get("rdf:type") {
                    let id = object_value
                        .get("@id")
                        .unwrap()
                        .as_str()
                        .unwrap_or_default()
                        .trim_start_matches("sho:");
                    let description = object_value
                        .get("description")
                        .unwrap()
                        .as_str()
                        .unwrap_or_default();
                    if object_type.get("@id").map_or(false, |v| v == "sho:Hazard") {
                        let has_category = object_value
                            .get("hasCategory")
                            .unwrap()
                            .as_str()
                            .unwrap_or_default()
                            .trim_start_matches("sho:");
                        hazards.push(HazardData {
                            description: description.to_owned(),
                            name: id.to_owned(),
                            category: has_category.to_owned(),
                        });
                        categories_hazards
                            .entry(has_category.to_owned())
                            .or_insert_with(Vec::new)
                            .push(id.to_owned());
                    } else if object_type
                        .get("@id")
                        .map_or(false, |v| v == "sho:Category")
                    {
                        categories.push(CategoryData {
                            description: description.to_owned(),
                            name: id.to_owned(),
                            hazards: Vec::new(),
                        });
                    }
                }
            }
        }

        categories.iter_mut().for_each(|category| {
            category.hazards = categories_hazards.get(&category.name).unwrap().to_owned();
        });

        context.insert("hazards".to_string(), Value::from_serializable(&hazards));
        context.insert(
            "categories".to_string(),
            Value::from_serializable(&categories),
        );

        let (files, dirs) = self.project_structure(output_path);

        (files, dirs, context)
    }

    fn create_source(&self) -> Source {
        super::build_source(&[self.template])
    }
}
