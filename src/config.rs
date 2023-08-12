// TODO: break this up into multiple modules (database, web_framework, etc.) and then have a config module that imports them all and builds the config object
use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::{
    error::Error,
    path::{Path, PathBuf},
    str::FromStr,
    vec,
};

use slug::slugify;
use strum::{EnumProperty, EnumString, EnumVariantNames, IntoEnumIterator, VariantNames};

use crate::{module::Module, toml_parser::TomlTemplate, StackTemplate};

type NpmDeps = Vec<Module>;
type CargoDeps = Vec<Module>;
type ComposerDeps = Vec<Module>;

#[derive(Debug, Clone)]
pub struct ScaffoldConfig {
    title: String,
    root_dir: PathBuf,
    languages: Vec<Language>,
    web_frameworks: Option<Vec<WebFramework>>,
    test_frameworks: Vec<TestFramework>,
    db: Option<Database>,
    db_client: Option<DbClient>,
    cms: Option<CMS>,
    linters: Vec<Linter>,
    formatters: Vec<Formatter>,
    npm_deps: Option<NpmDeps>,
    composer_deps: Option<ComposerDeps>,
    cargo_deps: Option<CargoDeps>,
    subfolders: Option<Vec<PathBuf>>,
}

impl ScaffoldConfig {
    pub fn new(options: UserOptions) -> Self {
        let toml = TomlTemplate::new(&options.stack.get_path());
        let dependencies = toml.get_dependencies();

        //TODO: refactor out base/common properties
        match options.stack {
            StackTemplate::SSRJS => Self {
                title: options.app_name,
                root_dir: PathBuf::from(&options.output_dir),
                languages: vec![Language::TypeScript, Language::JavaScript], // make sure lang is installed
                web_frameworks: Some(vec![WebFramework::Astro, WebFramework::Vue]), // ? handle from TOML?
                test_frameworks: vec![TestFramework::Vitest], // ? handle from TOML?
                db: options.db,
                db_client: None,
                cms: None,
                linters: vec![],
                formatters: vec![],
                npm_deps: dependencies.get("npm").unwrap_or(&None).clone(),
                composer_deps: dependencies.get("composer").unwrap().clone(),
                cargo_deps: dependencies.get("cargo").unwrap().clone(),
                subfolders: toml.get_subfolders().cloned(),
            },
            _ => Self {
                title: options.app_name,
                root_dir: PathBuf::from(&options.output_dir),
                languages: vec![],
                web_frameworks: None,
                test_frameworks: vec![],
                db: None,
                db_client: None,
                cms: None,
                linters: vec![],
                formatters: vec![],
                npm_deps: dependencies.get("npm").unwrap().clone(),
                composer_deps: dependencies.get("composer").unwrap().clone(),
                cargo_deps: dependencies.get("cargo").unwrap().clone(),
                subfolders: toml.get_subfolders().cloned(),
            },
        }
    }

    pub fn get_subfolders(&self) -> Option<&Vec<PathBuf>> {
        self.subfolders.as_ref()
    }

    pub fn get_root_dir(&self) -> &PathBuf {
        &self.root_dir
    }

    pub fn get_npm_deps(&self) -> &Option<NpmDeps> {
        &self.npm_deps
    }

    pub fn get_cargo_deps(&self) -> &Option<CargoDeps> {
        &self.cargo_deps
    }

    pub fn get_composer_deps(&self) -> &Option<ComposerDeps> {
        &self.composer_deps
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
    let mut stacks = StackTemplate::iter();
    let prompt_labels = stacks
        .clone()
        .map(|s| s.get_str("Label").unwrap())
        .collect::<Vec<_>>();
    let stack_template_index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What stack would you like to use?")
        .items(&prompt_labels)
        .interact()
        .expect("Failed to get stack selection from user");
    stacks.nth(stack_template_index).unwrap()
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
