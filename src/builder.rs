use crate::{config::ScaffoldConfig, dialogue::StackTemplate, file_system, module::Module};
use std::{
    env,
    fmt::{self, Debug, Formatter},
    path::PathBuf,
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
        std::env::set_current_dir(&self.config.root_dir).expect("Failed to set current directory");

        // Copy templates and manifests
        let _ = self.pre_install_commands();

        let install_commands = self.get_install_commands();

        for mut command in install_commands {
            println!("Running command: {:?}", command);
            let output = command.output().expect("Failed to execute command");
            println!("->> STDOUT: {}", String::from_utf8_lossy(&output.stdout));
            println!("->> STDERR: {}", String::from_utf8_lossy(&output.stderr));
        }

        self.set_npm_scripts();

        self.post_install_commands();
        //TODO? Can set custom cargo scripts or makefiles if needed down the road
        // self.set_cargo_scripts();

        //TODO:  (build templates / update manifests / containerize as needed)
        // TODO: improve logging during build
        // copy templates over to project
    }

    fn make_folders(&self) {
        println!("Making folders...");
        let root_dir = &self.config.root_dir;
        if let Some(folders) = &self.config.subfolders {
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
        println!("->> NPM: {:?}", self.config.npm_deps);
        commands.append(&mut self.generate_npm_cmds());
        println!("->> CARGO: {:?}", self.config.cargo_deps);
        commands.append(&mut self.generate_cargo_cmds());
        println!("->> LINTERS: {:?}", self.config.linters);
        commands.append(&mut self.generate_linter_cmds());
        println!("->> DATABASE_CLIENT: {:?}", self.config.db);
        commands.append(&mut self.generate_db_client_cmds());

        commands
    }

    fn generate_init_cmds(&self) -> Vec<Command> {
        let mut commands = vec![];
        if self.config.cargo_deps.is_some() {
            let mut cargo_init = Command::new("cargo");
            cargo_init.arg("init");
            commands.push(cargo_init);
        }
        if self.config.npm_deps.is_some() {
            let mut npm_init = Command::new("bun");
            npm_init.args(&["init", "-y"]);
            commands.push(npm_init);

            let mut package_name = Command::new("npm");
            package_name.args(&["pkg", "set", "name", &self.config.user_options.app_name]);
            commands.push(package_name);
        }

        commands
    }

    fn generate_npm_cmds(&self) -> Vec<Command> {
        let mut commands = vec![];
        if let Some(npm_modules) = &self.config.npm_deps {
            for module in npm_modules {
                let mut command = Command::new("bun");
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

                if let Some(mut cmds) = self.generate_then_cmds(&module) {
                    commands.append(&mut cmds);
                }
            }
        }

        commands
    }

    fn generate_cargo_cmds(&self) -> Vec<Command> {
        let mut commands = vec![];
        if let Some(cargo_modules) = &self.config.cargo_deps {
            for module in cargo_modules {
                let mut command = Command::new("cargo");
                command.env("CARGO_NET_GIT_FETCH_WITH_CLI", "true");
                command.arg("add");

                if module.get_version() != "latest" {
                    command.arg(format!("{}@{}", module.get_name(), module.get_version()));
                } else {
                    command.arg(module.get_name());
                }

                if module.is_dev() {
                    command.arg("--dev");
                }

                if let Some(features) = module.get_features() {
                    command.arg("--features");
                    command.arg(features.join(","));
                }

                commands.push(command);

                if let Some(mut cmds) = self.generate_then_cmds(&module) {
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
        if let Some(npm_scripts) = &self.config.npm_scripts {
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

    fn generate_linter_cmds(&self) -> Vec<Command> {
        let mut commands = vec![];
        let linters = &self.config.linters;
        if linters.len() > 0 {
            for linter in linters {
                commands.append(&mut linter.get_install_commands());
            }
        }
        commands
    }

    fn generate_db_client_cmds(&self) -> Vec<Command> {
        let mut commands = vec![];
        if let Some(db_client) = &self.config.db_client {
            commands.append(&mut db_client.get_install_commands(&self.config));
        }
        commands
    }

    fn pre_install_commands(&self) -> std::io::Result<()> {
        println!("Running pre-install commands...");

        let pre_install_path = self.template_dir().join("before_install");
        file_system::copy_dir_all(pre_install_path, env::current_dir().unwrap())
    }

    fn post_install_commands(&self) {
        println!("Running post-install commands...");
        let stack = &self.config.user_options.stack;
        // stack specific commands
        match stack {
            StackTemplate::RSAPI | StackTemplate::TSAPI => {
                if self.config.user_options.template_engine {
                    println!("->> installing template engine");
                    match stack {
                        StackTemplate::TSAPI => {
                            let mut command = Command::new("bun");
                            command.args(&["add", "handlebars"]);
                            command.output().expect("Failed to execute command");
                        }
                        StackTemplate::RSAPI => {
                            let mut command = Command::new("cargo");
                            command.args(&["add", "handlebars"]);
                            command.output().expect("Failed to execute command");
                        }
                        _ => (),
                    }
                }
                if self.config.user_options.spa {
                    println!("->> Creating Vue/Vite SPA");
                    let mut command = Command::new("bun");
                    command.args(&["create", "vue@latest"]);
                    command
                        .spawn()
                        .unwrap()
                        .wait()
                        .expect("Failed to execute command");
                }
            }
            _ => (),
        }

        // general commands
        println!("->> Copying Post-install templates...");
        let post_install_path = self.template_dir().join("after_install");
        file_system::copy_dir_all(post_install_path, env::current_dir().unwrap())
            .expect("unable to copy dir");
    }

    fn template_dir(&self) -> PathBuf {
        env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .join(self.config.user_options.stack.get_path().parent().unwrap())
    }
}

impl Debug for ProjectBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Project Builder")
            .field("config", &self.config)
            .finish()
    }
}
