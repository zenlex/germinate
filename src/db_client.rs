use std::process::Command;

use crate::{
    config::{Language, ScaffoldConfig},
    dialogue::Database,
};

#[derive(Debug, Clone)]
pub enum DbClient {
    Diesel,       // Rust ORM
    Sqlx,         // Rust typed SQL
    Prisma,       // TS ORM
    Slonik,       // TS typed SQL
    BetterSqlite, // Node SQLite3 driver
    MongoDb,      // Node/Rust MongoDB driver
    Mongoose,     // Node MongoDB ORM
}

impl DbClient {
    pub fn get_install_commands(&self, config: &ScaffoldConfig) -> Vec<Command> {
        let db = config
            .db
            .as_ref()
            .expect("No database specified for client");
        match self {
            DbClient::Diesel => {
                let mut command = Command::new("cargo");
                command.args(&["add", "diesel"]);

                match db {
                    Database::Postgres => command.args(&["--features", "postgres"]),
                    Database::Sqlite => command.args(&["--features", "sqlite"]),
                    Database::Mongo => &mut command,
                };
                vec![command]
            }
            DbClient::Sqlx => {
                let mut command = Command::new("cargo");
                command.args(&["add", "sqlx"]);

                let mut features = String::from("runtime-tokio,tls-native-tls,time,migrate");

                let additional_features = match db {
                    Database::Postgres => "postgres",
                    Database::Sqlite => "sqlite",
                    Database::Mongo => "",
                };

                if !additional_features.is_empty() {
                    features = format!("{},{}", features, additional_features);
                }

                command.arg("--features").arg(features);

                vec![command]
            }
            DbClient::Prisma => {
                let mut command = Command::new("bun");
                command.args(&["add", "prisma", "--dev"]);

                let init_command = Command::new("bunx");
                command.args(&["prisma", "init"]);

                vec![command, init_command]
            }
            DbClient::Slonik => {
                if let Database::Postgres = db {
                    let mut command = Command::new("bun");
                    command.args(&["add", "slonik", "--dev"]);
                    vec![command]
                } else {
                    panic!("No Slonik support for non-Postgres databases")
                }
            }
            DbClient::BetterSqlite => {
                let mut command = Command::new("bun");
                command.args(&["add", "better-sqlite3"]);
                vec![command]
            }
            DbClient::MongoDb => {
                if config.has_language(&Language::Rust) {
                    let mut command = Command::new("cargo");
                    command.args(&["add", "mongodb"]);
                    vec![command]
                } else {
                    let mut command = Command::new("bun");
                    command.args(&["add", "mongodb"]);
                    vec![command]
                }
            }
            DbClient::Mongoose => {
                let mut command = Command::new("bun");
                command.args(&["add", "mongoose"]);
                vec![command]
            }
        }
    }
}
