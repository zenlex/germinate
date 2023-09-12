use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::test_frameworks::TestFramework;
use dialoguer::{theme::ColorfulTheme, Input, MultiSelect, Select};
use slug::slugify;
use strum::{EnumIter, EnumProperty, EnumString, EnumVariantNames, IntoEnumIterator, VariantNames};

#[derive(Debug, Clone, EnumVariantNames, EnumString, EnumIter, EnumProperty)]
pub enum StackTemplate {
    #[strum(props(Label = "SSR TypeScript"))]
    TSWEB,
    #[strum(props(Label = "Laravel with Vue + Inertia"))]
    Laravel,
    #[strum(props(Label = "TypeScript API, optional frontend"))]
    TSAPI,
    #[strum(props(Label = "Rust Web App, optional frontend"))]
    RSWEB,
    #[strum(props(Label = "Rust CLI Tool"))]
    RSCLI,
    #[strum(props(Label = "TypeScript CLI Tool"))]
    TSCLI,
}

impl StackTemplate {
    pub fn get_path(&self) -> PathBuf {
        match self {
            Self::TSWEB => PathBuf::from("templates/tsweb/stack_template.toml"),
            Self::Laravel => PathBuf::from("templates/laravel/stack_template.toml"),
            Self::RSWEB => PathBuf::from("templates/rsweb/stack_template.toml"),
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
    pub test_frameworks: Vec<TestFramework>,
    pub containers: bool,
}

pub fn get_user_config() -> Result<UserOptions, std::io::Error> {
    let stack = get_stack();
    let (spa, template_engine) = get_frontend(&stack);
    let app_name = get_app_name();
    let output_dir = slugify(&app_name);
    let db = get_db();
    let orm = match &db {
        Some(_) => get_orm(),
        None => false,
    };
    let test_frameworks = test_frameworks_prompt();
    let containers = containers_prompt();

    let user_config = UserOptions {
        app_name,
        stack,
        output_dir: Path::new(&output_dir).to_path_buf(),
        db,
        orm,
        spa,
        template_engine,
        test_frameworks,
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
    let use_db = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Would you like to use a database?")
        .items(&["Yes", "No"])
        .interact()
        .ok()?;

    let db = if use_db == 0 {
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
    Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Would you like to use an ORM?")
        .items(&["Yes", "No"])
        .interact()
        .expect("Failed to get ORM selection from user")
        == 0
}

fn get_frontend(stack: &StackTemplate) -> (bool, bool) {
    match stack {
        StackTemplate::RSWEB | StackTemplate::TSAPI => {
            let spa = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Would you like to use a SPA?")
                .items(&["Yes", "No"])
                .interact()
                .expect("Failed to get SPA selection from user")
                == 0;

            let template = match spa {
                true => false,
                false => {
                    Select::with_theme(&ColorfulTheme::default())
                        .with_prompt("Would you like to use a frontend template engine?")
                        .items(&["Yes", "No"])
                        .interact()
                        .expect("Failed to get frontend template selection from user")
                        == 0
                }
            };
            return (spa, template);
        }
        _ => return (false, false),
    }
}
fn test_frameworks_prompt() -> Vec<TestFramework> {
    let test_frameworks = TestFramework::VARIANTS;
    let prompt_labels = test_frameworks.clone();
    let test_framework_indexes = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("What test frameworks would you like to use?")
        .items(&prompt_labels)
        .interact()
        .expect("Failed to get test frameworks selection from user");
    let mut results = vec![];
    for index in test_framework_indexes {
        results.push(
            <TestFramework as FromStr>::from_str(test_frameworks[index])
                .expect("Invalid test framework name"),
        );
    }

    results
}

fn containers_prompt() -> bool {
    Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Would you like to use Docker containers?")
        .items(&["Yes", "No"])
        .interact()
        .expect("Failed to get containers selection from user")
        == 0
}
