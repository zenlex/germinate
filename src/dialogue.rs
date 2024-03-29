use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use slug::slugify;
use strum::{EnumIter, EnumProperty, EnumString, EnumVariantNames, IntoEnumIterator, VariantNames};

#[derive(Debug, Clone, EnumVariantNames, EnumString, EnumIter, EnumProperty)]
pub enum StackTemplate {
    #[strum(props(Label = "TypeScript Web App"))]
    TSAPI,
    #[strum(props(Label = "Rust Web App"))]
    RSAPI,
    #[strum(props(Label = "Rust CLI Tool"))]
    RSCLI,
    #[strum(props(Label = "TypeScript CLI Tool"))]
    TSCLI,
}

impl StackTemplate {
    pub fn get_path(&self) -> PathBuf {
        match self {
            Self::RSAPI => PathBuf::from("templates/rsapi/stack_template.toml"),
            Self::TSCLI => PathBuf::from("templates/tscli/stack_template.toml"),
            Self::RSCLI => PathBuf::from("templates/rscli/stack_template.toml"),
            Self::TSAPI => PathBuf::from("templates/tsapi/stack_template.toml"),
        }
    }
}
#[derive(Debug, Clone)]
pub struct UserOptions {
    pub stack: StackTemplate,
    pub output_dir: PathBuf,
    pub app_name: String,
    pub db: Option<Database>,
    pub orm: bool,
    pub spa: bool,
    pub template_engine: bool,
    pub containers: bool,
}

pub fn get_user_config() -> Result<UserOptions, std::io::Error> {
    let stack = get_stack();
    let (spa, template_engine) = get_frontend(&stack);
    let app_name = get_app_name();
    let output_dir = slugify(&app_name);
    let db = get_db();
    let orm = match &db {
        Some(db) => match db {
            Database::Mongo => match stack {
                StackTemplate::RSCLI | StackTemplate::RSAPI => false,
                _ => get_orm(),
            },
            _ => get_orm(),
        },
        None => false,
    };

    let containers = match stack {
        StackTemplate::RSAPI | StackTemplate::TSAPI => containers_prompt(),
        _ => false,
    };

    let user_config = UserOptions {
        app_name,
        stack,
        output_dir: Path::new(&output_dir).to_path_buf(),
        db,
        orm,
        spa,
        template_engine,
        containers,
    };

    println!("->> User Config generated: {:?}", user_config);

    Ok(user_config)
}

#[derive(Debug, Clone, EnumVariantNames, EnumString)]
pub enum Database {
    Postgres,
    Mongo,
    Sqlite,
}

fn get_app_name() -> String {
    Input::<String>::new()
        .with_prompt("What is the name of your project?")
        .interact_text()
        .unwrap()
}

fn get_stack() -> StackTemplate {
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

fn get_db() -> Option<Database> {
    let use_db = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Would you like to use a database?")
        .interact()
        .ok()?;

    let db = if use_db {
        Some(get_db_platform())
    } else {
        None
    };

    db
}

fn get_db_platform() -> Database {
    let db_options = Database::VARIANTS;
    let db_index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What database would you like to use?")
        .items(&db_options)
        .interact()
        .expect("Failed to get db selection from user");
    <Database as FromStr>::from_str(db_options[db_index]).expect("Invalid db name")
}

fn get_orm() -> bool {
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Would you like to use an ORM?")
        .interact()
        .expect("Failed to get ORM selection from user")
}

fn get_frontend(stack: &StackTemplate) -> (bool, bool) {
    match stack {
        StackTemplate::RSAPI | StackTemplate::TSAPI => {
            let spa = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Would you like to use a SPA?")
                .interact()
                .expect("Failed to get SPA selection from user");

            let template = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Would you like to use a frontend template engine?")
                .interact()
                .expect("Failed to get template engine selection from user");
            return (spa, template);
        }
        _ => return (false, false),
    }
}

fn containers_prompt() -> bool {
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Would you like to use Docker containers?")
        .interact()
        .expect("Failed to get containers selection from user")
}
