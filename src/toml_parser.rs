use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use crate::config::PackageScripts;
use crate::module::Module;
use toml::{map::Map, Table, Value};

type Dependencies = HashMap<String, Option<Vec<Module>>>;
type Scripts = HashMap<String, PackageScripts>;
#[derive(Debug, Clone)]
pub struct TomlTemplate {
    subfolders: Option<Vec<PathBuf>>,
    scripts: Option<Scripts>,
    dependencies: Dependencies,
}

impl TomlTemplate {
    pub fn new(path: &Path) -> Self {
        let table = Self::get_table(path);
        let subfolders = Self::parse_subfolders(&table);
        let scripts = Self::parse_scripts(&table);
        let dependencies = Self::parse_deps(&table);

        Self {
            subfolders,
            scripts,
            dependencies,
        }
    }

    pub fn get_subfolders(&self) -> Option<&Vec<PathBuf>> {
        self.subfolders.as_ref()
    }

    pub fn get_scripts(&self) -> Option<&Scripts> {
        self.scripts.as_ref()
    }

    pub fn get_dependencies(&self) -> &Dependencies {
        &self.dependencies
    }

    fn parse_deps(table: &Table) -> Dependencies {
        let deps = match table.get("deps") {
            Some(deps) => deps.as_table().expect("Error parsing dependencies"),
            None => panic!("No deps keys found in TOML template file"),
        };

        let package_managers = vec!["npm", "cargo"];
        let parsed_deps = Self::fetch_deps(package_managers, deps);

        parsed_deps
    }

    fn fetch_deps(keys: Vec<&str>, deps: &Table) -> Dependencies {
        let mut results = HashMap::new();
        keys.iter().for_each(|key| {
            let packages = match deps.get(*key) {
                Some(entries) => {
                    let entries = entries
                        .as_array()
                        .expect(format!("Error retrieving {} dependencies", key).as_str());
                    println!("Collecting {} dependencies", key);
                    Some(Self::format_deps(entries))
                }
                None => None,
            };
            results.insert(key.to_string(), packages);
        });
        results
    }

    fn format_deps(table: &Vec<Value>) -> Vec<Module> {
        table
            .iter()
            .map(|dep| {
                let dep = dep.as_table().expect("Error collecting deps");
                let name = match dep.get("name") {
                    Some(name) => name.as_str().expect("Error parsing name"),
                    None => panic!("Error parsing name"),
                };

                //TODO: add semver crate to allow for parsing semver ranges
                let version = match dep.get("version") {
                    Some(version) => version.as_str().expect("Error parsing version"),
                    None => "latest",
                };

                let dev = match dep.get("dev") {
                    Some(dev) => dev.as_bool().expect("Error parsing dev"),
                    None => false,
                };

                let then = match dep.get("then") {
                    Some(cmds) => {
                        let cmds = cmds
                            .as_array()
                            .expect("Error parsing then array")
                            .iter()
                            .map(|arr| arr.as_array().expect("Error parsing cmd"))
                            .map(|cmd_arr| {
                                cmd_arr
                                    .iter()
                                    .map(|arg| arg.as_str().expect("Error parsing arg").to_string())
                                    .collect()
                            })
                            .collect();
                        Some(cmds)
                    }
                    None => None,
                };

                let features = match dep.get("features") {
                    Some(features) => Some(
                        features
                            .as_array()
                            .expect("Error parsing dev")
                            .iter()
                            .map(|feature| {
                                feature.as_str().expect("Error parsing feature").to_string()
                            })
                            .collect(),
                    ),
                    None => None,
                };
                Module::new(name.to_string(), version.to_string(), dev, then, features)
            })
            .collect()
    }

    fn parse_scripts(table: &Map<String, Value>) -> Option<Scripts> {
        let package_managers = vec!["npm", "cargo"];
        match table.get("scripts") {
            Some(scripts) => {
                let scripts_table = scripts.as_table().expect("Error parsing dependencies");
                Some(Self::format_scripts(package_managers, scripts_table))
            }
            None => None,
        }
    }

    fn format_scripts(keys: Vec<&str>, table: &Table) -> Scripts {
        let mut results = HashMap::new();
        keys.iter().for_each(|key| {
            match table.get(*key) {
                Some(entries) => {
                    let mut scripts = HashMap::new();
                    entries
                        .as_table()
                        .expect("Error extracting npm scripts table")
                        .iter()
                        .for_each(|(key, val)| {
                            scripts.insert(
                                key.to_string(),
                                val.as_str().expect("Error parsing script").to_string(),
                            );
                        });
                    results.insert(key.to_string(), scripts);
                }
                None => (),
            };
        });

        results
    }

    fn parse_subfolders(table: &Map<String, Value>) -> Option<Vec<PathBuf>> {
        match table.get("subfolders") {
            Some(subfolders) => {
                let subfolders = subfolders.as_table().expect("Error parsing subfolders");
                let mut paths: Vec<PathBuf> = vec![];
                let path = Path::new("");
                let mut child_paths: Vec<PathBuf> = subfolders
                    .iter()
                    .flat_map(|child| Self::get_sub_paths(child, &path))
                    .collect();
                paths.append(&mut child_paths);
                Some(paths)
            }
            None => {
                println!("No subfolders key in table");
                None
            }
        }
    }

    fn get_sub_paths((name, children): (&String, &toml::Value), path: &Path) -> Vec<PathBuf> {
        let mut paths: Vec<PathBuf> = vec![];
        let children = children
            .as_table()
            .expect("Error parsing subfolder children");
        if children.is_empty() {
            paths.push(path.join(name));
        } else {
            let path = path.join(name);
            let mut child_paths: Vec<PathBuf> = children
                .iter()
                .flat_map(|child| Self::get_sub_paths(child, &path))
                .collect();
            paths.append(&mut child_paths);
        }

        paths
    }

    //TODO? this requires templates folder to live in the same directory as the binary, could add a config/cli flag
    fn get_table(path: &Path) -> Table {
        let template_str =
            fs::read_to_string(path).expect(&format!("Error reading file: {}", path.display()));
        let table = template_str.parse::<Table>().expect("Error parsing toml");
        table
    }
}

#[cfg(test)]

pub mod tests {
    use toml::map::Map;

    use super::*;

    #[test]
    fn test_parse_toml() {
        let path = Path::new("test/__mocks__/_test.toml");
        let template = TomlTemplate::new(path);

        dbg!(&template);
        let deps_table = template.get_dependencies();
        let scripts_table = template.get_scripts().expect("No scripts extracted");
        let subfolders = template.get_subfolders().expect("No subfolders extracted");

        assert!(deps_table.contains_key("npm"));
        assert!(deps_table.contains_key("cargo"));

        assert!(scripts_table.contains_key("npm"));
        assert!(scripts_table.contains_key("cargo"));

        assert!(!subfolders.is_empty());
    }

    #[test]
    fn test_parse_deps() {
        let deps_table = get_deps_table();
        let parsed_deps = TomlTemplate::fetch_deps(vec!["npm", "cargo"], &deps_table);
        assert!(parsed_deps.contains_key("npm"));
        assert!(parsed_deps.contains_key("cargo"));
    }

    #[test]
    fn fetch_npm_deps() {
        let deps_table = get_deps_table();
        let parsed_deps = TomlTemplate::fetch_deps(vec!["npm"], &deps_table);
        assert!(parsed_deps.contains_key("npm"));

        let npm_deps = &parsed_deps["npm"].as_ref().unwrap();
        assert!(npm_deps.iter().any(|dep| dep.name == "test_npm_dep_min"));
        assert!(npm_deps
            .iter()
            .any(|dep| dep.name == "test_npm_dev_dep_full"));

        let full_dev_dep = npm_deps
            .iter()
            .find(|dep| dep.name == "test_npm_dev_dep_full")
            .expect("Error finding dep");
        assert_eq!(full_dev_dep.version, "^1.0.0");
        assert_eq!(full_dev_dep.dev, true);

        let then_cmds = full_dev_dep.then.as_ref().expect("Error getting then cmds");
        assert_eq!(then_cmds.len(), 2);
        assert_eq!(then_cmds[0].len(), 1);
        assert_eq!(then_cmds[1].len(), 3);
        assert_eq!(then_cmds[0][0], "naked_command");
        assert_eq!(then_cmds[1][0], "command_with_args");
        assert_eq!(then_cmds[1][1], "arg1");
        assert_eq!(then_cmds[1][2], "arg2");
    }

    #[test]
    fn fetch_cargo_deps() {
        let deps_table = get_deps_table();
        let parsed_deps = TomlTemplate::fetch_deps(vec!["cargo"], &deps_table);
        assert!(parsed_deps.contains_key("cargo"));

        let cargo_deps = &parsed_deps["cargo"].as_ref().unwrap();
        assert!(cargo_deps
            .iter()
            .any(|dep| dep.name == "test_cargo_dep_min"));
        assert!(cargo_deps
            .iter()
            .any(|dep| dep.name == "test_cargo_dev_dep_full"));

        let full_dev_dep = cargo_deps
            .iter()
            .find(|dep| dep.name == "test_cargo_dev_dep_full")
            .expect("Error finding dep");
        assert_eq!(full_dev_dep.version, "^1.0.0");
        assert_eq!(full_dev_dep.dev, true);

        let then_cmds = full_dev_dep.then.as_ref().expect("Error getting then cmds");
        assert_eq!(then_cmds.len(), 2);
        assert_eq!(then_cmds[0].len(), 1);
        assert_eq!(then_cmds[1].len(), 3);
        assert_eq!(then_cmds[0][0], "naked_command");
        assert_eq!(then_cmds[1][0], "command_with_args");
        assert_eq!(then_cmds[1][1], "arg1");
        assert_eq!(then_cmds[1][2], "arg2");
    }

    #[test]
    fn extract_npm_scripts() {
        let path = Path::new("test/__mocks__/_test.toml");
        let table = TomlTemplate::get_table(path);
        let parsed_scripts = TomlTemplate::parse_scripts(&table).expect("Error parsing deps");

        assert!(parsed_scripts.contains_key("npm"));
        let npm_scripts = parsed_scripts
            .get("npm")
            .expect("Error getting npm scripts");

        assert_eq!(npm_scripts["dev"], "test dev");
        assert_eq!(npm_scripts["start"], "test prod");
        assert_eq!(npm_scripts["build"], "test build");
        assert_eq!(npm_scripts["preview"], "test preview");
    }

    #[test]
    fn extract_subfolders() {
        let path = Path::new("test/__mocks__/_test.toml");
        let table = TomlTemplate::get_table(path);
        let folder_tree = TomlTemplate::parse_subfolders(&table);

        assert!(folder_tree.is_some());
        let folder_tree = folder_tree.unwrap();
        dbg!(&folder_tree);

        assert_eq!(folder_tree.len(), 9);
        assert!(folder_tree.contains(&PathBuf::from("l0foo")));
        assert!(folder_tree.contains(&PathBuf::from("l0bar")));
        assert!(folder_tree.contains(&PathBuf::from("l0baz")));
        assert!(folder_tree.contains(&PathBuf::from("single_depth/l1foo")));
        assert!(folder_tree.contains(&PathBuf::from("single_depth/l1bar")));
        assert!(folder_tree.contains(&PathBuf::from("single_depth/l1baz")));
        assert!(folder_tree.contains(&PathBuf::from("single_depth/double_depth/l2foo")));
        assert!(folder_tree.contains(&PathBuf::from("single_depth/double_depth/l2bar")));
        assert!(folder_tree.contains(&PathBuf::from("single_depth/double_depth/l2baz")));
    }

    // Helpers
    fn get_deps_table() -> Map<String, Value> {
        let path = Path::new("test/__mocks__/_test.toml");
        let toml_table = TomlTemplate::get_table(path);
        let deps_table = match toml_table.get("deps") {
            Some(deps) => deps.as_table().expect("Error parsing dependencies"),
            None => panic!("No deps keys found in TOML template file"),
        };
        deps_table.to_owned()
    }
}
