// TODO: break this up into multiple modules (database, web_framework, etc.) and then have a config module that imports them all and builds the config object
use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::{
    collections::HashMap,
    error::Error,
    path::{Path, PathBuf},
    str::FromStr,
};

use slug::slugify;
use strum::{EnumString, EnumVariantNames, VariantNames};

use crate::module::Module;
use crate::StackTemplate;

#[derive(Debug, Clone)]
pub struct ScaffoldConfig {
    languages: Vec<Language>,
    web_frameworks: Vec<WebFramework>,
    test_frameworks: Vec<TestFramework>,
    db: Option<Database>,
    db_client: Option<DbClient>,
    cms: Option<CMS>,
    linters: Vec<Linter>,
    formatters: Vec<Formatter>,
    npm_deps: HashMap<String, Module>,
    composer_deps: HashMap<String, Module>,
    cargo_deps: HashMap<String, Module>,
}

impl ScaffoldConfig {
    pub fn new(options: UserOptions) -> Self {
        match options.stack {
            StackTemplate::SSRJS => Self {
                languages: vec![Language::TypeScript, Language::JavaScript], // make sure lang is installed
                web_frameworks: vec![WebFramework::Astro, WebFramework::Vue],
                test_frameworks: vec![TestFramework::Vitest],
                db: options.db,
                db_client: None,
                cms: None,
                linters: vec![],
                formatters: vec![],
                npm_deps: HashMap::new(),
                composer_deps: HashMap::new(),
                cargo_deps: HashMap::new(),
            },
            _ => Self {
                languages: vec![],
                web_frameworks: vec![],
                test_frameworks: vec![],
                db: None,
                db_client: None,
                cms: None,
                linters: vec![],
                formatters: vec![],
                npm_deps: HashMap::new(),
                composer_deps: HashMap::new(),
                cargo_deps: HashMap::new(),
            },
        }
    }
}

#[derive(Debug)]
pub struct UserOptions {
    pub stack: StackTemplate,
    pub output_dir: PathBuf,
    pub app_name: String,
    pub db: Option<Database>,
}

pub fn get_user_config() -> Result<UserOptions, std::io::Error> {
    let app_name = Input::<String>::new()
        .with_prompt("What is the name of your project?")
        .interact_text()?;

    let output_dir = slugify(&app_name);
    let stack = stack_prompts();
    let db = db_prompts();

    //TODO: handle follow up questions based on stack choice

    let user_config = UserOptions {
        app_name,
        stack,
        output_dir: Path::new(&output_dir).to_path_buf(),
        db,
    };
    println!("->> User Config generated: {:?}", user_config);

    Ok(user_config)
}

#[derive(Debug, Clone, EnumVariantNames, EnumString)]
pub enum Database {
    Postgres,
}

#[derive(Debug, Clone)]
enum Language {
    Rust,
    JavaScript,
    TypeScript,
    PHP,
}

#[derive(Debug, Clone)]
enum WebFramework {
    Axum,
    Express,
    Astro,
    Laravel,
    Vue,
    Dioxus,
}

#[derive(Debug, Clone)]
enum CMS {
    Filament,
    Strapi,
}

#[derive(Debug, Clone)]
enum DbClient {
    Diesel,
    Sqlx,
    Prisma,
    Slonik,
}

#[derive(Debug, Clone)]
enum TestFramework {
    Jest,
    Vitest,
    PHPUnit,
    Pest,
    Playwright,
    Dusk,
}

#[derive(Debug, Clone)]
enum Linter {
    ESLint,
    PHPStan,
    Stylelint,
}

#[derive(Debug, Clone)]
enum Formatter {
    ESLint,
    PhpCsFixer,
    Pint,
    Rustfmt,
}

trait BuildDep {
    fn build(&self) -> Result<(), Box<dyn Error>>;
    fn destroy(&self) -> Result<(), Box<dyn Error>>;
}

fn stack_prompts() -> StackTemplate {
    let stack_options = StackTemplate::VARIANTS;
    let stack_template_index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What stack would you like to use?")
        .items(&stack_options)
        .interact()
        .expect("Failed to get stack selection from user");
    let stack_name = stack_options[stack_template_index];
    <StackTemplate as FromStr>::from_str(stack_name).expect("Invalid stack name")
}

fn db_prompts() -> Option<Database> {
    let use_db = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Would you like to use a database?")
        .items(&["Yes", "No"])
        .interact()
        .ok()?;

    let db = if use_db == 0 {
        Some(get_db_from_user())
    } else {
        None
    };

    db
}

fn get_db_from_user() -> Database {
    let db_options = Database::VARIANTS;
    let db_index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What database would you like to use?")
        .items(&db_options)
        .interact()
        .expect("Failed to get db selection from user");
    <Database as FromStr>::from_str(db_options[db_index]).expect("Invalid db name")
}
