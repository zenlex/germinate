use crate::config::ScaffoldConfig;
use std::fmt::{self, Debug, Formatter};

pub struct ProjectBuilder {
    config: ScaffoldConfig,
}

#[allow(unused)]
impl ProjectBuilder {
    pub fn new(config: ScaffoldConfig) -> Self {
        Self { config }
    }

    pub fn get_config(&self) -> &ScaffoldConfig {
        &self.config
    }

    pub fn build(&self) {
        println!("Building project...");
        // create project folders
        self.make_folders()
        // generate install commands
        // run install commands
        // (build templates as needed)
        // copy templates
    }

    fn make_folders(&self) {
        println!("Making folders...");
        let root_dir = self.config.get_root_dir();
        if let Some(folders) = self.config.get_subfolders() {
            for folder in folders {
                let full_path = root_dir.join(folder);
                println!("Creating folder: {:?}", full_path);

                std::fs::create_dir_all(full_path).unwrap();
            }
        }
    }
}

impl Debug for ProjectBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Project Builder")
            .field("config", &self.config)
            .finish()
    }
}
