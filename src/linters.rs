use std::process::Command;

#[derive(Debug, Clone)]
pub enum Linter {
    ESLint,
    Stylelint,
    Clippy,
}

impl Linter {
    pub fn get_install_commands(&self) -> Vec<Command> {
        match self {
            Linter::ESLint => {
                let mut command = Command::new("bun");
                command.arg("add").arg("eslint").arg("--dev");
                vec![command]
            }
            Linter::Stylelint => {
                let mut command = Command::new("bun");
                command.arg("add").arg("stylelint").arg("--dev");
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
