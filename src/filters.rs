use minijinja::{Error, State};

pub(crate) fn hypens_to_underscores(_state: &State, value: String) -> Result<String, Error> {
    Ok(value.replace("-", "_"))
}
