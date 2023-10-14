use crate::{
    config::ScaffoldConfig, container::ContainerBuilder, dialogue::StackTemplate, file_system,
    linters::Linter, module,
};
use std::{collections::HashMap, env, io::Result, path::PathBuf, process::Command, vec};

pub struct ProjectBuilder {}

impl ProjectBuilder {
    pub fn build(config: &ScaffoldConfig) {
        println!("Building project...");
        make_folders(&config.root_dir, &config.subfolders);
        std::env::set_current_dir(&config.root_dir).expect("Failed to set current directory");

        pre_install_commands(config).expect("Failed to run pre-install commands");
        install_commands(config).expect("Failed to run install commands");
        post_install_commands(config).expect("Failed to run post-install commands");
    }
}

fn pre_install_commands(config: &ScaffoldConfig) -> Result<()> {
    println!("Running pre-install commands...");
    let pre_install_path = config.template_dir.join("before_install");
    file_system::copy_dir_all(pre_install_path, env::current_dir().unwrap())
}

fn install_commands(config: &ScaffoldConfig) -> Result<()> {
    println!("Installing dependencies...");
    for mut command in get_install_commands(config) {
        println!("Running command: {:?}", command);
        let output = command.output().expect("Failed to execute command");
        println!("->> STDOUT: {}", String::from_utf8_lossy(&output.stdout));
        println!("->> STDERR: {}", String::from_utf8_lossy(&output.stderr));
    }
    Ok(())
}

fn post_install_commands(config: &ScaffoldConfig) -> Result<()> {
    println!("Running post-install commands...");

    // stack specific commands
    let stack = &config.user_options.stack;
    match stack {
        StackTemplate::RSAPI | StackTemplate::TSAPI => {
            if config.user_options.template_engine {
                println!("->> installing template engine");
                match stack {
                    StackTemplate::TSAPI => {
                        let mut command = Command::new("bun");
                        command.args(&["add", "handlebars", "--features", "dir_source"]);
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
            if config.user_options.spa {
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
    println!("->> Removing boilerplate files...");
    match stack {
        StackTemplate::TSAPI | StackTemplate::TSCLI => {
            std::fs::remove_file("index.ts").ok();
        }
        _ => {}
    }
    println!("->> Copying Post-install templates...");
    let post_install_path = config.template_dir.join("after_install");
    file_system::copy_dir_all(post_install_path, env::current_dir().unwrap())
        .expect("unable to copy dir");

    if config.containers {
        ContainerBuilder::new(config).build();
    }

    if let Some(npm_scripts) = &config.npm_scripts {
        set_npm_scripts(npm_scripts);
    }

    create_repo();

    Ok(())
}

fn make_folders(root_dir: &PathBuf, subfolders: &Option<Vec<PathBuf>>) {
    println!("Making folders...");
    if let Some(folders) = subfolders {
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

fn get_install_commands(config: &ScaffoldConfig) -> Vec<Command> {
    println!("Queueing install commands...");
    let mut commands = vec![];

    commands.append(&mut generate_init_cmds(config));

    if let Some(npm_deps) = &config.npm_deps {
        commands.append(&mut module::get_npm_cmds(npm_deps));
    }

    if let Some(cargo_deps) = &config.cargo_deps {
        commands.append(&mut module::get_cargo_cmds(cargo_deps));
    }

    if let Some(db_client) = &config.db_client {
        commands.append(&mut db_client.get_install_commands(config));
    }

    commands.append(&mut generate_linter_cmds(&config.linters));

    commands
}

fn generate_init_cmds(config: &ScaffoldConfig) -> Vec<Command> {
    let mut commands = vec![];

    if config.cargo_deps.is_some() {
        println!("Generating Cargo init...");
        let mut cargo_init = Command::new("cargo");
        cargo_init.arg("init");
        commands.push(cargo_init);
    }
    if config.npm_deps.is_some() {
        println!("Generating NPM init...");
        let mut npm_init = Command::new("bun");
        npm_init.args(&["init", "-y"]);
        commands.push(npm_init);

        let mut package_name = Command::new("npm");
        package_name.args(&["pkg", "set", "name", &config.user_options.app_name]);
        commands.push(package_name);
    }

    commands
}

fn set_npm_scripts(scripts: &HashMap<String, String>) {
    println!("Setting NPM scripts...");
    for (name, script) in scripts {
        let mut command = Command::new("npm");
        command
            .args(&["pkg", "set"])
            .arg(format!("scripts.{}={}", name, script))
            .output()
            .expect("Failed to set npm scripts");
    }
}

fn generate_linter_cmds(linters: &Vec<Linter>) -> Vec<Command> {
    let mut commands = vec![];
    for linter in linters {
        commands.append(&mut linter.get_install_commands());
    }
    commands
}

fn create_repo() {
    println!("Creating git repo...");
    let mut command = Command::new("git");
    command.args(&["init"]);
    command.output().expect("Failed to create git repo");

    let mut command = Command::new("git");
    command.args(&["checkout", "-b", "main"]);
    command.output().expect("Failed to create main branch");

    println!("Creating initial commit...");
    let mut command = Command::new("git");
    command.args(&["add", "."]);
    command.output().expect("Failed to add files to git repo");

    let mut command = Command::new("git");
    command.args(&["commit", "-m", "Initial commit"]);
    command.output().expect("Failed to create initial commit");
}
