mod config;

use clap::{Parser, ValueEnum};
use config::{Database, UserOptions};
use dialoguer::{console::Term, theme::ColorfulTheme, Input, Select};
use slug::slugify;
use std::{
    io::Error,
    path::{Path, PathBuf},
    str::FromStr,
};
use strum::{EnumString, EnumVariantNames, VariantNames};

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
    let user_config = get_user_config().unwrap();
    println!("->> User Config generated: {:?}", user_config);

    // create config
    let app_config = ScaffoldConfig::new(user_config.stack, user_config.db);
    // run scaffolding engine
    // return success/errors
}

fn get_user_config() -> Result<UserOptions, Error> {
    //TODO: encapsulate each build/prompt step here into it's own function and probably move to the config module
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

    let use_db = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Would you like to use a database?")
        .items(&["Yes", "No"])
        .interact()?;

    let db = if use_db == 0 {
        Some(get_db_from_user())
    } else {
        None
    };

    //TODO: handle follow up questions based on stack choice (e.g. php extensions, db, etc. )

    Ok(UserOptions {
        app_name: app_name.clone(),
        stack: <StackTemplate as FromStr>::from_str(stack_name).expect("Invalid stack name"),
        output_dir: Path::new(&output_dir).to_path_buf(),
        db,
    })
}

fn get_db_from_user() -> Database {
    let db_options = Database::VARIANTS;
    let db_index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What database would you like to use?")
        .items(&db_options)
        .interact()?;
    <Database as FromStr>::from_str(db_options[db_index]).expect("Invalid db name")
}
