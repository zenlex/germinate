use crate::config::ScaffoldConfig;
use std::{
    fmt::{self, Debug, Formatter},
    process::Command,
    vec,
};

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
        self.make_folders();
        // generate install commands
        let commands = self.get_install_commands();
        dbg!(&commands);
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

    fn get_install_commands(&self) -> Vec<Command> {
        println!("Queueing install commands...");
        println!("->> NPM: {:?}", self.config.get_npm_deps());
        let mut commands = vec![];
        commands.append(&mut self.generate_npm_cmds());
        println!("->> CARGO: {:?}", self.config.get_cargo_deps());
        commands.append(&mut self.generate_cargo_cmds());
        println!("->> COMPOSER: {:?}", self.config.get_composer_deps());
        commands.append(&mut self.generate_composer_cmds());

        commands
    }

    fn generate_npm_cmds(&self) -> Vec<Command> {
        let mut commands = vec![];
        for module in self.config.get_npm_deps() {
            let mut command = Command::new("npm");
            command.arg("install");

            if module.get_version() != "latest" {
                command.arg(format!("{}@{}", module.get_name(), module.get_version()));
            } else {
                command.arg(module.get_name());
            }

            if module.is_dev() {
                command.arg("--save-dev");
            }

            commands.push(command);

            if let Some(post_install_cmds) = module.get_then() {
                for cmd in post_install_cmds {
                    let mut command = Command::new(cmd[0].clone());
                    for arg in &cmd[1..] {
                        command.arg(arg);
                    }
                    commands.push(command);
                }
            }
        }

        commands
    }

    fn generate_cargo_cmds(&self) -> Vec<Command> {
        let mut commands = vec![];
        todo!();
        commands
    }

    fn generate_composer_cmds(&self) -> Vec<Command> {
        let mut commands = vec![];
        todo!();
        commands
    }
}

impl Debug for ProjectBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Project Builder")
            .field("config", &self.config)
            .finish()
    }
}
