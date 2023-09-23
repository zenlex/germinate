use serde::{Deserialize, Serialize};
use std::{
    env,
    fs::{self},
};

use crate::{config::ScaffoldConfig, dialogue::Database, file_system};

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
        copy_docker_files(&self.config);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DockerVariables {
    app_name: String,
    deps_name: String, // the app name prefix of the generated deps files to remove in prod for containers
    postgres: bool,
}

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
            postgres: matches!(db.as_ref().unwrap(), Database::Postgres),
        }
    }
}

fn copy_docker_files(config: &ScaffoldConfig) {
    generate_dockerfile(config);
    generate_docker_compose(config);
    println!("Copying Docker files...");
    let pre_install_path = config.template_dir.join("docker");
    dbg!(&pre_install_path);
    file_system::copy_dir_all(pre_install_path, env::current_dir().unwrap())
        .expect("unable to copy dir");

    println!("Removing templates...");
    fs::remove_file("dockerfile.template").expect("Failed to remove dockerfile template");
}

fn generate_dockerfile(config: &ScaffoldConfig) {
    println!("Generating Docker files...");
    let template_path = config
        .template_dir
        .join("docker")
        .join("dockerfile.template");

    let template = fs::read_to_string(template_path).expect("Failed to read dockerfile template");
    crate::template_generator::render_to_file(
        &template,
        &DockerVariables::new(&config.db),
        &mut fs::File::create("Dockerfile").unwrap(),
    )
    .expect("Failed to render dockerfile template");
}

fn generate_docker_compose(config: &ScaffoldConfig) {
    println!("Generating docker-compose files...");
    let template_path = config
        .template_dir
        .join("docker")
        .join("docker-compose.yml.template");

    dbg!(&template_path);
    let template = fs::read_to_string(template_path).expect("Failed to read dockerfile template");
    crate::template_generator::render_to_file(
        &template,
        &DockerVariables::new(&config.db),
        &mut fs::File::create("docker-compose.yml").unwrap(),
    )
    .expect("Failed to render dockerfile template");
}
