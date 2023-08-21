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
                let mut command = Command::new("npm");
                command.arg("install").arg("jest").arg("--save-dev");
                vec![command]
            }
            TestFramework::Vitest => {
                let mut command = Command::new("npm");
                command.arg("install").arg("vitest").arg("--save-dev");
                vec![command]
            }
            TestFramework::Pest => {
                let mut command = Command::new("composer");
                command.arg("require").arg("pestphp/pest").arg("--dev");
                vec![command]
            }
            TestFramework::Playwright => {
                let mut command = Command::new("npm");
                command.arg("install").arg("playwright").arg("--save-dev");
                vec![command]
            }
        }
    }
}
