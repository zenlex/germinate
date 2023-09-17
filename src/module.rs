use std::process::Command;

#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub version: String,
    pub dev: bool,
    pub features: Option<Vec<String>>,
    pub then: Option<ThenCommands>,
}

pub type ThenCommands = Vec<Vec<String>>;

impl Module {
    pub fn new(
        name: String,
        version: String,
        dev: bool,
        then: Option<ThenCommands>,
        features: Option<Vec<String>>,
    ) -> Self {
        Self {
            name,
            version,
            dev,
            features,
            then,
        }
    }
}

pub fn get_npm_cmds(npm_modules: &Vec<Module>) -> Vec<Command> {
    let mut commands = vec![];
    for module in npm_modules {
        let mut command = Command::new("bun");
        command.arg("add");

        if module.version != "latest" {
            command.arg(format!("{}@{}", module.name, module.version));
        } else {
            command.arg(&module.name);
        }

        if module.dev {
            command.arg("--dev");
        }

        commands.push(command);

        if let Some(then_commands) = &module.then {
            commands.append(&mut generate_then_cmds(&then_commands));
        }
    }

    commands
}

pub fn get_cargo_cmds(cargo_modules: &Vec<Module>) -> Vec<Command> {
    let mut commands = vec![];
    for module in cargo_modules {
        let mut command = Command::new("cargo");
        command.env("CARGO_NET_GIT_FETCH_WITH_CLI", "true");
        command.arg("add");

        if module.version != "latest" {
            command.arg(format!("{}@{}", module.name, module.version));
        } else {
            command.arg(&module.name);
        }

        if module.dev {
            command.arg("--dev");
        }

        if let Some(features) = &module.features {
            command.arg("--features");
            command.arg(features.join(","));
        }

        commands.push(command);

        if let Some(then_commands) = &module.then {
            commands.append(&mut generate_then_cmds(then_commands))
        }
    }
    commands
}

fn generate_then_cmds(then_commands: &ThenCommands) -> Vec<Command> {
    let mut commands = vec![];
    for cmd in then_commands {
        let mut command = Command::new(&cmd[0]);
        for arg in &cmd[1..] {
            command.arg(arg);
        }
        commands.push(command);
    }
    commands
}
