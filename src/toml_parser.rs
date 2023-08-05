use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use toml::Table;

use crate::module::Module;

#[allow(unused)]
pub struct TomlTemplate {
    title: String,
    subfolders: Option<Vec<Vec<String>>>,
    scripts: Option<Scripts>,
    dependencies: Option<Dependencies>,
}

type Dependencies = HashMap<String, Vec<Module>>;
type Scripts = HashMap<String, HashMap<String, String>>;

#[allow(unused)]
impl TomlTemplate {
    pub fn parse_deps(path: &Path) -> Option<Dependencies> {
        let table = Self::get_table(path).expect("Error parsing toml");
        let deps = match table.get("deps") {
            Some(deps) => deps.as_table().expect("Error parsing dependencies"),
            None => panic!("No deps key in table"),
        };

        let npm_deps: Vec<_> = match deps.get("npm") {
            Some(entries) => {
                entries
                    .as_array()
                    .expect("Error parsing npm dependencies")
                    .iter()
                    .map(|dep| {
                        let dep = dep.as_table().expect("Error parsing dep");
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
                                            .map(|arg| {
                                                arg.as_str().expect("Error parsing arg").to_string()
                                            })
                                            .collect()
                                    })
                                    .collect();
                                Some(cmds)
                            }
                            None => None,
                        };
                        Module::new(name.to_string(), version.to_string(), dev, then)
                    })
                    .collect()
            }
            None => Vec::new(),
        };

        let mut deps = HashMap::new();
        deps.insert("npm".to_string(), npm_deps);
        Some(deps)
    }

    fn parse_scripts(path: &Path) -> Option<Scripts> {
        let table = Self::get_table(path).expect("Error parsing toml");
        let scripts = match table.get("scripts") {
            Some(scripts) => scripts.as_table().expect("Error parsing dependencies"),
            None => panic!("No deps key in table"),
        };

        let npm_scripts = match scripts.get("npm") {
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
                scripts
            }
            None => HashMap::new(),
        };

        let mut scripts = HashMap::new();
        scripts.insert("npm".to_string(), npm_scripts);

        Some(scripts)
    }

    fn parse_subfolders(path: &Path) -> Option<Vec<PathBuf>> {
        let table = Self::get_table(path).expect("Error parsing toml");
        let subfolders = match table.get("subfolders") {
            Some(subfolders) => Some(subfolders.as_table().expect("Error parsing subfolders")),
            None => {
                println!("No subfolders key in table");
                None
            }
        };

        let mut paths: Vec<PathBuf> = vec![];
        if subfolders.is_some() {
            let subfolders = subfolders.unwrap();
            let path = Path::new("");
            let mut child_paths: Vec<PathBuf> = subfolders
                .iter()
                .flat_map(|child| Self::get_sub_paths(child, &path))
                .collect();
            paths.append(&mut child_paths);
        } else {
            println!("No subfolders key in table");
        }
        Some(paths)
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

    fn get_table(path: &Path) -> Option<Table> {
        let template_str = fs::read_to_string(path).expect("Error reading file");
        let table = template_str.parse::<Table>().expect("Error parsing toml");
        Some(table)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_parse_toml() {
        let path = Path::new("test/__mocks__/_test.toml");
        let template_str = fs::read_to_string(path).expect("Error reading file");
        let table = template_str.parse::<Table>().expect("Error parsing toml");
        assert_eq!(table["title"].as_str(), Some("toml_test_template"));
    }

    #[test]
    fn extract_npm_deps() {
        let path = Path::new("test/__mocks__/_test.toml");
        let parsed_deps = TomlTemplate::parse_deps(path).expect("Error parsing deps");
        assert!(parsed_deps.contains_key("npm"));

        let npm_deps = &parsed_deps["npm"];
        assert!(npm_deps
            .iter()
            .any(|dep| dep.get_name() == "test_npm_dep_min"));
        assert!(npm_deps
            .iter()
            .any(|dep| dep.get_name() == "test_npm_dev_dep_full"));

        let full_dev_dep = npm_deps
            .iter()
            .find(|dep| dep.get_name() == "test_npm_dev_dep_full")
            .expect("Error finding dep");
        assert_eq!(full_dev_dep.get_version(), "^1.0.0");
        assert_eq!(full_dev_dep.is_dev(), true);

        let then_cmds = full_dev_dep.get_then().expect("Error getting then cmds");
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
        let parsed_scripts = TomlTemplate::parse_scripts(path).expect("Error parsing deps");

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
        let folder_tree = TomlTemplate::parse_subfolders(path);

        assert!(folder_tree.is_some());
        let folder_tree = folder_tree.unwrap();

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
}
