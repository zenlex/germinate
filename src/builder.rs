use crate::{config::ScaffoldConfig, module::Module};
use std::{
    fmt::{self, Debug, Formatter},
    process::Command,
    vec,
};

pub struct ProjectBuilder {
    config: ScaffoldConfig,
}

impl ProjectBuilder {
    pub fn new(config: ScaffoldConfig) -> Self {
        Self { config }
    }

    pub fn build(&self) {
        println!("Building project...");
        self.make_folders();
        std::env::set_current_dir(self.config.get_root_dir())
            .expect("Failed to set current directory");

        let commands = self.get_install_commands();

        for mut command in commands {
            println!("Running command: {:?}", command);
            let output = command.output().expect("Failed to execute command");
            println!("->> STDOUT: {}", String::from_utf8_lossy(&output.stdout));
            println!("->> STDERR: {}", String::from_utf8_lossy(&output.stderr));
        }

        self.set_npm_scripts();
        self.set_composer_scripts();
        //TODO? Can set custom cargo scripts or makefiles if needed down the road
        // self.set_cargo_scripts();

        //TODO:  (build templates / update manifests / containerize as needed)
        // TODO: improve logging during build
        // copy templates over to project
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
        let mut commands = vec![];
        commands.append(&mut self.generate_init_cmds());
        println!("->> NPM: {:?}", self.config.get_npm_deps());
        commands.append(&mut self.generate_npm_cmds());
        println!("->> CARGO: {:?}", self.config.get_cargo_deps());
        commands.append(&mut self.generate_cargo_cmds());
        println!("->> COMPOSER: {:?}", self.config.get_composer_deps());
        commands.append(&mut self.generate_composer_cmds());
        println!("->> LINTERS: {:?}", self.config.get_linters());
        commands.append(&mut self.generate_linter_cmds());
        println!("->> FORMATTERS: {:?}", self.config.get_formatters());
        commands.append(&mut self.generate_formatter_cmds());

        commands
    }

    fn generate_init_cmds(&self) -> Vec<Command> {
        let mut commands = vec![];
        if self.config.get_npm_deps().is_some() {
            let mut npm_init = Command::new("npm");
            npm_init.arg("init").arg("-y");
            commands.push(npm_init);
        }
        if self.config.get_composer_deps().is_some() {
            //TODO: make non-interactive by passing --no-interaction + basic data flags
            let mut composer_init = Command::new("composer");
            composer_init.arg("init");
            commands.push(composer_init);
        }

        commands
    }

    fn generate_npm_cmds(&self) -> Vec<Command> {
        let mut commands = vec![];
        if let Some(npm_modules) = self.config.get_npm_deps() {
            for module in npm_modules {
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
        }

        commands
    }

    fn generate_cargo_cmds(&self) -> Vec<Command> {
        let mut commands = vec![];
        if let Some(cargo_modules) = self.config.get_cargo_deps() {
            for module in cargo_modules {
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
        }
        commands
    }

    fn generate_composer_cmds(&self) -> Vec<Command> {
        let mut commands = vec![];
        if let Some(composer_modules) = self.config.get_composer_deps() {
            for module in composer_modules {
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

    fn set_npm_scripts(&self) {
        if let Some(npm_scripts) = self.config.get_npm_scripts() {
            println!("Setting NPM scripts...");
            for (name, script) in npm_scripts {
                let mut command = Command::new("npm");
                command
                    .arg("pkg")
                    .arg("set")
                    .arg(format!("scripts.{}={}", name, script));
                println!("Running command: {:?}", command);
                let output = command.output().expect("Failed to execute command");
                println!("->> STDOUT: {}", String::from_utf8_lossy(&output.stdout));
                println!("->> STDERR: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
    }

    fn set_composer_scripts(&self) {
        if let Some(composer_scripts) = self.config.get_composer_scripts() {
            println!("Setting Composer scripts...");
            for (name, script) in composer_scripts {
                let mut command = Command::new("composer");
                command
                    .arg("config --")
                    .arg(format!("scripts.{}", name))
                    .arg(script);
                println!("Running command: {:?}", command);
                let output = command.output().expect("Failed to execute command");
                println!("->> STDOUT: {}", String::from_utf8_lossy(&output.stdout));
                println!("->> STDERR: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
    }

    fn generate_linter_cmds(&self) -> Vec<Command> {
        let mut commands = vec![];
        let linters = self.config.get_linters();
        if linters.len() > 0 {
            for linter in linters {
                commands.append(&mut linter.get_install_commands());
            }
        }
        commands
    }

    fn generate_formatter_cmds(&self) -> Vec<Command> {
        let mut commands = vec![];
        let formatters = self.config.get_formatters();
        if formatters.len() > 0 {
            for formatter in formatters {
                commands.append(&mut formatter.get_install_commands());
            }
        }
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
