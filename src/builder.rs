use crate::{config::ScaffoldConfig, module::Module};
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
            dbg!(&folders);
            for folder in folders {
                let full_path = root_dir.join(folder);
                println!("Creating folder: {:?}", full_path);
                std::fs::create_dir_all(&full_path)
                    .expect(format!("Failed to create folder: {:?}", &full_path).as_str());
            }
        } else {
            println!("Creating root folder only: {:?}", &root_dir);
            std::fs::create_dir_all(root_dir).unwrap();
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

            if let Some(mut cmds) = self.generate_then_cmds(module) {
                commands.append(&mut cmds);
            }
        }

        commands
    }

    fn generate_cargo_cmds(&self) -> Vec<Command> {
        let mut commands = vec![];
        for module in self.config.get_cargo_deps() {
            let mut command = Command::new("cargo");
            command.arg("add");

            if module.get_version() != "latest" {
                command.arg(format!("{}@{}", module.get_name(), module.get_version()));
            } else {
                command.arg(module.get_name());
            }

            if module.is_dev() {
                command.arg("--dev");
            }

            commands.push(command);

            if let Some(mut cmds) = self.generate_then_cmds(module) {
                commands.append(&mut cmds);
            }
        }

        commands
    }

    fn generate_composer_cmds(&self) -> Vec<Command> {
        let mut commands = vec![];
        for module in self.config.get_composer_deps() {
            let mut command = Command::new("composer");
            command.arg("require");

            if module.get_version() != "latest" {
                command.arg(format!("{}@{}", module.get_name(), module.get_version()));
            } else {
                command.arg(module.get_name());
            }

            if module.is_dev() {
                command.arg("--dev");
            }

            commands.push(command);

            if let Some(mut cmds) = self.generate_then_cmds(module) {
                commands.append(&mut cmds);
            }
        }

        commands
    }

    fn generate_then_cmds(&self, module: &Module) -> Option<Vec<Command>> {
        match module.get_then() {
            Some(cmds) => {
                let mut commands = vec![];
                for cmd in cmds {
                    let mut command = Command::new(&cmd[0]);
                    for arg in &cmd[1..] {
                        command.arg(arg);
                    }
                    commands.push(command);
                }
                Some(commands)
            }
            None => None,
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
