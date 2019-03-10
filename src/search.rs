use crate::data::PrefixBase;

pub enum SearchError {
    NotImplemented,
}

pub struct Path {
    words: Vec<String>,
}

fn find_path(a: &str, b: &str, prefixes: &PrefixBase) -> Result<Path,SearchError> {
    Err(SearchError::NotImplemented)
}
