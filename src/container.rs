use serde::{Deserialize, Serialize};
use std::{env, fs};

use crate::{config::ScaffoldConfig, dialogue::Database, template_generator};

pub struct ContainerBuilder {
    config: ScaffoldConfig,
}

impl ContainerBuilder {
    pub fn new(config: &ScaffoldConfig) -> Self {
        Self {
            config: config.clone(),
        }
    }

    pub fn build(&self) {
        generate_dockerfiles(&self.config);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DockerVariables {
    app_name: String,
    deps_name: String, // the app name prefix of the generated deps files to remove in prod for containers
    database: bool,
    postgres: bool,
    mongo: bool,
    sqlite: bool,
}

impl crate::template_generator::TemplateData for DockerVariables {}

impl DockerVariables {
    pub fn new(db: &Option<Database>) -> Self {
        let kebab_name = env::current_dir()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let snake_name = kebab_name.replace("-", "_");
        Self {
            app_name: kebab_name,
            deps_name: snake_name,
            database: db.is_some(),
            postgres: db
                .as_ref()
                .is_some_and(|db| matches!(db, Database::Postgres)),
            mongo: db.as_ref().is_some_and(|db| matches!(db, Database::Mongo)),
            sqlite: db.as_ref().is_some_and(|db| matches!(db, Database::Sqlite)),
        }
    }
}

fn generate_dockerfiles(config: &ScaffoldConfig) {
    println!("Generating Docker files...");

    template_generator::generate_dir(
        config.template_dir.join("docker"),
        env::current_dir().unwrap().join("docker"),
        &DockerVariables::new(&config.db),
        true,
    );

    println!("Moving docker-compose.yml to project root...");
    fs::copy(
        env::current_dir()
            .unwrap()
            .join("docker/docker-compose.yml"),
        env::current_dir().unwrap().join("docker-compose.yml"),
    )
    .expect("Failed to copy docker-compose.yml to project root");
    fs::remove_file(
        env::current_dir()
            .unwrap()
            .join("docker/docker-compose.yml"),
    )
    .expect("Failed to remove docker-compose.yml from docker directory");
}
