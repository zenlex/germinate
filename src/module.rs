use std::process::Command;

#[derive(Debug, Clone)]
pub struct Module {
    name: String,
    version: String,
    dev: bool,
    features: Option<Vec<String>>,
    then: Option<ThenCommands>,
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

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_version(&self) -> &str {
        &self.version
    }

    pub fn is_dev(&self) -> bool {
        self.dev
    }

    pub fn get_features(&self) -> Option<&Vec<String>> {
        self.features.as_ref()
    }

    pub fn get_then(&self) -> Option<&ThenCommands> {
        self.then.as_ref()
    }
}

pub fn get_npm_cmds(npm_modules: &Vec<Module>) -> Vec<Command> {
    let mut commands = vec![];
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

        if let Some(then_commands) = module.get_then() {
            commands.append(&mut generate_then_cmds(then_commands));
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

        if let Some(then_commands) = module.get_then() {
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
