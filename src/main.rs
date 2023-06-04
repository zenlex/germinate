mod config;

use clap::{Parser, ValueEnum};
use config::UserOptions;
use dialoguer::{console::Term, theme::ColorfulTheme, Input, Select};
use slug::slugify;
use std::{
    io::Error,
    path::{Path, PathBuf},
    str::FromStr,
};
use strum::{EnumString, EnumVariantNames, VariantNames};

#[derive(Debug, Clone, EnumVariantNames, EnumString)]
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
    // TODO: parse command line args (need to make them optional in a logical way)
    // let args = ScaffoldArgs::parse();
    // println!("{:?}", args);

    // get user options
    let user_config = get_user_config().unwrap();
    println!("->> User Config generated: {:?}", user_config);

    // create config
    // run scaffolding engine
    // return success/errors
}

fn get_user_config() -> Result<config::UserOptions, Error> {
    let app_name = Input::<String>::new()
        .with_prompt("What is the name of your project?")
        .interact_text()?;

    let output_dir = slugify(&app_name);

    let stack_options = StackTemplate::VARIANTS;
    let stack_template_index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What stack would you like to use?")
        .items(&stack_options)
        .interact()?;
    let stack_name = stack_options[stack_template_index];

    Ok(config::UserOptions {
        app_name: app_name.clone(),
        stack: StackTemplate::from_str(stack_name).expect("Invalid stack name"),
        output_dir: Path::new(&output_dir).to_path_buf(),
    })
}
