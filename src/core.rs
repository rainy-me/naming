use regex::Regex;
use std::collections::HashMap;
use std::fmt;

use crate::api;
use api::TreeResponse;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Naming {
    Pascal,
    Camel,
    Snake,
    Kebab,
    Lower,
    Upper,
    Unknown,
}

impl fmt::Display for Naming {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub type LanguageCount = HashMap<Naming, u64>;
pub type Info = HashMap<String, LanguageCount>;

pub fn get_naming_count(repo: &str) -> Result<Info, reqwest::Error> {
    let mut info: Info = HashMap::new();
    let resp: TreeResponse = api::get_tree(repo).unwrap();
    let filenames: Vec<Option<&str>> = resp
        .tree
        .iter()
        .filter(|file| file.r#type == "blob")
        .map(|file| file.path.rsplitn(2, '/').next())
        .collect::<Vec<_>>();

    for maybe_filename in filenames.iter() {
        if let Some(filename) = maybe_filename {
            let mut parts = filename.split('.');
            let maybe_name = parts.nth(0);
            let maybe_suffix = parts.last();

            if let (Some(name), Some(suffix)) = (maybe_name, maybe_suffix) {
                if name != "" {
                    let naming_style = get_naming_style(name);
                    let mut language: LanguageCount = HashMap::new();
                    language.insert(naming_style, 0);

                    *info
                        .entry(suffix.to_owned())
                        .or_insert(language)
                        .entry(naming_style)
                        .or_insert(0) += 1;
                }
            }
        }
    }
    Ok(info)
}

fn get_naming_style(naming: &str) -> Naming {
    lazy_static! {
        static ref PASCAL_RE: Regex = Regex::new("^[A-Z][a-z]+(?:[A-Z][a-z]+)+$").unwrap();
        static ref CAMEL_RE: Regex = Regex::new("^[a-z]+(?:[A-Z][a-z]+)+$").unwrap();
        static ref SNAKE_RE: Regex = Regex::new("^[a-z]+(?:_[a-z]+)+$").unwrap();
        static ref KEBAB_RE: Regex = Regex::new("^[a-z]+(?:-[a-z]+)+$").unwrap();
        static ref LOWER_RE: Regex = Regex::new("^[a-z]+$").unwrap();
        static ref UPPER_RE: Regex = Regex::new("^[A-Z]+$").unwrap();
    }
    let mut style = Naming::Unknown;
    if PASCAL_RE.is_match(naming) {
        style = Naming::Pascal;
        return style;
    }
    if CAMEL_RE.is_match(naming) {
        style = Naming::Camel;
        return style;
    }
    if SNAKE_RE.is_match(naming) {
        style = Naming::Snake;
        return style;
    }
    if KEBAB_RE.is_match(naming) {
        style = Naming::Kebab;
        return style;
    }
    if LOWER_RE.is_match(naming) {
        style = Naming::Lower;
        return style;
    }
    if UPPER_RE.is_match(naming) {
        style = Naming::Upper;
        return style;
    }
    style
}

#[test]
fn test_get_naming_style() {
    assert_eq!(get_naming_style("PascalCase"), Naming::Pascal);
    assert_eq!(get_naming_style("camelCase"), Naming::Camel);
    assert_eq!(get_naming_style("snake_case"), Naming::Snake);
    assert_eq!(get_naming_style("kebab-case"), Naming::Kebab);
    assert_eq!(get_naming_style("lower"), Naming::Lower);
    assert_eq!(get_naming_style("UPPER"), Naming::Upper);
    assert_eq!(get_naming_style("1234"), Naming::Unknown);
    assert_eq!(get_naming_style("forwardRef-component"), Naming::Unknown);
}
