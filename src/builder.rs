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
    }
}

impl Debug for ProjectBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Project Builder")
            .field("config", &self.config)
            .finish()
    }
}
