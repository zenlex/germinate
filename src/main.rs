mod config;

use clap::{Parser, ValueEnum};
use std::path::PathBuf;
use strum::{EnumString, EnumVariantNames};

use crate::config::ScaffoldConfig;

#[derive(Debug, Clone, EnumVariantNames, EnumString, ValueEnum)]
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
    stack: Option<StackTemplate>,
    #[arg(short, long)]
    output_dir: Option<PathBuf>,
}
fn main() {
    // TODO: parse command line args (need to make them optional in a logical way and decide on precedence)
    let args = ScaffoldArgs::parse();
    println!("{:?}", args);

    // get user options
    let user_config = config::get_user_config().unwrap();

    // create config
    let app_config = ScaffoldConfig::new(user_config);
    println!("->> APP_CONFIG: {:?}", app_config);
    // run scaffolding engine

    // return success/errors
}
