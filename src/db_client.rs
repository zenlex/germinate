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
                command.arg("add").arg("diesel");

                match db {
                    Database::Postgres => command.arg("--features").arg("postgres"),
                    Database::Sqlite => command.arg("--features").arg("sqlite"),
                    Database::Mongo => &mut command,
                };
                vec![command]
            }
            DbClient::Sqlx => {
                let mut command = Command::new("cargo");
                command.arg("add").arg("sqlx");

                let mut features = String::from("runtime-tokio,tls-native-tls");

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
                command.arg("add").arg("prisma").arg("--dev");

                let init_command = Command::new("bunx");
                command.arg("prisma").arg("init");

                vec![command, init_command]
            }
            DbClient::Slonik => {
                if let Database::Postgres = db {
                    let mut command = Command::new("bun");
                    command.arg("add").arg("slonik").arg("--dev");
                    vec![command]
                } else {
                    panic!("No Slonik support for non-Postgres databases")
                }
            }
            DbClient::BetterSqlite => {
                let mut command = Command::new("bun");
                command.arg("add").arg("better-sqlite3");
                vec![command]
            }
            DbClient::MongoDb => {
                if config.has_language(&Language::Rust) {
                    let mut command = Command::new("cargo");
                    command.arg("add").arg("mongodb");
                    vec![command]
                } else {
                    let mut command = Command::new("bun");
                    command.arg("add").arg("mongodb");
                    vec![command]
                }
            }
            DbClient::Mongoose => {
                let mut command = Command::new("bun");
                command.arg("add").arg("mongoose");
                vec![command]
            }
        }
    }
}
