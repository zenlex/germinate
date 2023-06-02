mod config;

use std::path::PathBuf;

use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, ValueEnum)]
pub enum StackTemplate {
    SSRJS,   // Server Side Rendered JavaScript/TypeScript
    SPAJS,   // Single Page Application JavaScript/TypeScript
    Laravel, // PHP Laravel with Vue + Inertia
    TSCLI,   // TypeScript CLI / Native App
    RSCLI,   // Rust CLI / Native App
}
#[derive(Debug, Parser)]
#[command(author, version, about, long_about= None)]
pub struct ScaffoldArgs {
    #[arg(short, long)]
    stack: StackTemplate,
    #[arg(short, long, default_value = "./")]
    output_dir: Option<PathBuf>,
}
fn main() {
    // parse command line args
    let args = ScaffoldArgs::parse();
    println!("{:?}", args);
    // create config
    // run scaffolding engine
    // return success/errors
}
