use crate::{config::ScaffoldConfig, toml_parser::TomlTemplate};
use std::fmt::{self, Debug, Formatter};

pub struct ProjectBuilder {
    config: ScaffoldConfig,
}

impl ProjectBuilder {
    pub fn new(config: ScaffoldConfig) -> Self {
        Self { config }
    }

    pub fn get_config(&self) -> &ScaffoldConfig {
        &self.config
    }

    pub fn build(&self) {
        println!("Building project");
        dbg!(&self.config);
    }
}

impl Debug for ProjectBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Project Builder")
            .field("config", &self.config)
            .finish()
    }
}
