use std::process::Command;

#[derive(Debug, Clone)]
pub enum Formatter {
    Pint,
    Rustfmt,
}

impl Formatter {
    pub fn get_install_commands(&self) -> Vec<Command> {
        match self {
            Formatter::Pint => {
                let mut command = Command::new("npm");
                command.arg("install").arg("pint").arg("--save-dev");
                vec![command]
            }
            Formatter::Rustfmt => {
                let mut commands = vec![];

                let mut command = Command::new("rustup");
                command.arg("update");
                commands.push(command);

                let mut command = Command::new("rustup");
                command.arg("component").arg("add").arg("rustfmt");
                commands.push(command);

                commands
            }
        }
    }
}
