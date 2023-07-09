use std::fs;
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
    fn test_module() {
        let module = Module::new("astro".to_string(), "0.1.0".to_string(), true, None);
        println!("{:?}", module);
    }

    #[test]
    fn test_parse_toml() {
        let template_str = fs::read_to_string("templates/ssrjs/ssrjs.toml").unwrap();
        let table = template_str.parse::<Table>().unwrap();
        assert_eq!(table["title"].as_str(), Some("ssrjs"));
        println!("{:?}", table);
    }
}
