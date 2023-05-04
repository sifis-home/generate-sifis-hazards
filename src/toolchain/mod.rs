pub mod external;
pub mod rust;

pub(crate) use external::*;
pub(crate) use rust::*;

macro_rules! builtin_templates {
    ($root:expr => $(($name:expr, $template:expr)),+) => {
        [
        $(
            (
                $name,
                include_str!(concat!(env!("CARGO_MANIFEST_DIR"),"/templates/", $root, "/", $template)),
            )
        ),+
        ]
    }
}

fn build_source(templates: &[(&str, &str)]) -> minijinja::Source {
    let mut source = minijinja::Source::new();
    for (name, src) in templates {
        source
            .add_template(*name, *src)
            .expect("Internal error, built-in template");
    }

    source
}

pub(crate) use builtin_templates;
