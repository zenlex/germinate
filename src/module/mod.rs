use std::{collections::HashMap, fs, path::Path};
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

type Dependencies = HashMap<String, Vec<Module>>;
type Scripts = HashMap<String, Vec<String>>;
struct TomlTemplate {
    title: String,
    subfolders: Option<Vec<Vec<String>>>,
    scripts: Option<Scripts>,
    dependencies: Option<Dependencies>,
}

impl TomlTemplate {
    pub fn parse_deps(path: &Path) -> Option<Dependencies> {
        let template_str = fs::read_to_string(path).expect("Error reading file");
        let table = template_str.parse::<Table>().expect("Error parsing toml");
        let deps = table["dependencies"]
            .as_table()
            .expect("Error parsing dependencies");

        let npm_deps: Vec<_> = deps["npm"]
            .as_array()
            .expect("Error parsing npm dependencies")
            .iter()
            .map(|dep| {
                let dep = dep.as_table().expect("Error parsing dep");
                let name = dep["name"].as_str().expect("Error parsing name");

                //TODO: add semver crate to allow for parsing semver ranges
                let version = if dep.contains_key("version") {
                    dep["version"].as_str().expect("Error parsing version")
                } else {
                    "latest"
                };

                let dev = if dep.contains_key("dev") {
                    dep["dev"].as_bool().expect("Error parsing dev")
                } else {
                    false
                };

                let then = if dep.contains_key("then") {
                    Some(
                        dep["then"]
                            .as_array()
                            .unwrap_or(&toml::value::Array::new())
                            .iter()
                            .map(|then| then.to_string())
                            .collect(),
                    )
                } else {
                    None
                };
                Module::new(name.to_string(), version.to_string(), dev, then)
            })
            .collect();

        let mut deps = HashMap::new();
        deps.insert("npm".to_string(), npm_deps);
        Some(deps)
    }
}

pub struct NpmDeps {
    deps: Vec<Module>,
}

impl NpmDeps {
    pub fn new() -> Self {
        Self { deps: Vec::new() }
    }

    pub fn add(&mut self, module: Module) {
        self.deps.push(module);
    }

    pub fn get(&self) -> Vec<Module> {
        self.deps.clone()
    }
}

// TODO add builders for composer and cargo deps
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
    }

    #[test]
    fn extract_npm_deps() {
        //TODO: make a test template file and assert the results
        let path = Path::new("templates/ssrjs/ssrjs.toml");
        let parsed_deps = TomlTemplate::parse_deps(path).expect("Error parsing deps");
        assert_eq!(parsed_deps.contains_key("npm"), true);
    }
}
