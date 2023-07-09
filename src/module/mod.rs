use std::{fs, path::Path};
use toml::Table;
#[derive(Debug, Clone)]
pub struct Module {
    name: String,
    version: String,
    dev: bool,
    then: Option<Vec<String>>,
}

impl Module {
    pub fn new(name: String, version: String, dev: bool, then: Option<Vec<String>>) -> Self {
        Self {
            name,
            version,
            dev,
            then,
        }
    }
}

struct TomlTemplate {
    title: String,
    subfolders: Option<Vec<Vec<String>>>,
    scripts: Option<Vec<Vec<String>>>,
    dependencies: Option<Vec<Vec<Module>>>,
}

pub struct NpmDeps {
    deps: Vec<Module>,
}

// read toml file to string
// parse toml string to table
// create module structs for each dep
// construct TomlTemplate struct from table
// ? refactor TomlTemplate construction to deserialize traits for toml? (maybe not worth it)
#[cfg(test)]
pub mod tests {

    use super::*;

    #[test]
    fn test_parse_toml() {
        let path = Path::new("templates/ssrjs/ssrjs.toml");
        let template_str = fs::read_to_string(path).expect("Error reading file");
        let table = template_str.parse::<Table>().expect("Error parsing toml");
        assert_eq!(table["title"].as_str(), Some("ssrjs"));
        println!("{:?}", table);
    }
}
