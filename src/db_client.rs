use std::process::Command;

#[derive(Debug, Clone)]
pub enum DbClient {
    Diesel,       // Rust ORM
    Sqlx,         // Rust typed SQL
    Prisma,       // TS ORM
    Slonik,       // TS typed SQL
    BetterSqlite, // Node SQLite3 driver
    MongoDb,      // Node/Rust/PHP MongoDB driver
    Mongoose,     // Node MongoDB ORM
}

impl DbClient {
    pub fn get_install_commands(&self) -> Vec<Command> {
        todo!();
    }
}
