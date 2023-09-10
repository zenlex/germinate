use std::process::Command;

use strum::{EnumString, EnumVariantNames};

#[derive(Debug, Clone, EnumVariantNames, EnumString)]
pub enum TestFramework {
    Jest,
    Vitest,
    Pest,
    Playwright,
}

impl TestFramework {
    pub fn get_install_commands(&self) -> Vec<Command> {
        match self {
            TestFramework::Jest => {
                let mut command = Command::new("bun");
                command.arg("add").arg("jest").arg("--dev");
                vec![command]
            }
            TestFramework::Vitest => {
                let mut command = Command::new("bun");
                command.arg("add").arg("vitest").arg("--dev");
                vec![command]
            }
            TestFramework::Pest => {
                let mut command = Command::new("composer");
                command.arg("require").arg("pestphp/pest").arg("--dev");
                vec![command]
            }
            TestFramework::Playwright => {
                let mut command = Command::new("bun");
                command.arg("add").arg("playwright").arg("--dev");
                vec![command]
            }
        }
    }
}
