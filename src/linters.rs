use std::process::Command;

#[derive(Debug, Clone)]
pub enum Linter {
    ESLint,
    Larastan,
    Stylelint,
    Clippy,
}

impl Linter {
    pub fn get_install_commands(&self) -> Vec<Command> {
        match self {
            Linter::ESLint => {
                let mut command = Command::new("npm");
                command.arg("install").arg("eslint").arg("--save-dev");
                vec![command]
            }
            Linter::Larastan => {
                let mut command = Command::new("composer");
                command
                    .arg("require")
                    .arg("nunomaduro/larastan")
                    .arg("--dev");
                vec![command]
            }
            Linter::Stylelint => {
                let mut command = Command::new("npm");
                command.arg("install").arg("stylelint").arg("--save-dev");
                vec![command]
            }
            Linter::Clippy => {
                let mut commands = vec![];

                let mut command = Command::new("rustup");
                command.arg("update");
                commands.push(command);

                let mut command = Command::new("rustup");
                command.arg("component").arg("add").arg("clippy");
                commands.push(command);

                commands
            }
        }
    }
}
