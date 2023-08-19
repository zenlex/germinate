use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use dialoguer::{theme::ColorfulTheme, Input, MultiSelect, Select};
use slug::slugify;
use strum::{EnumProperty, EnumString, EnumVariantNames, IntoEnumIterator, VariantNames};

use crate::StackTemplate;

#[derive(Debug)]
pub struct UserOptions {
    pub stack: StackTemplate,
    pub output_dir: PathBuf,
    pub app_name: String,
    pub db: Option<Database>,
    pub orm: bool,
    pub cms: bool,
    pub test_frameworks: Vec<TestFramework>,
}

pub fn get_user_config() -> Result<UserOptions, std::io::Error> {
    let app_name = get_app_name();
    let output_dir = slugify(&app_name);
    let stack = get_stack();
    let db = get_db();
    let orm = match &db {
        Some(_) => get_orm(),
        None => false,
    };

    let cms = get_cms();
    let test_frameworks = test_frameworks_prompt();

    let user_config = UserOptions {
        app_name,
        stack,
        output_dir: Path::new(&output_dir).to_path_buf(),
        db,
        orm,
        cms,
        test_frameworks,
    };

    println!("->> User Config generated: {:?}", user_config);

    Ok(user_config)
}

#[derive(Debug, Clone, EnumVariantNames, EnumString)]
pub enum Database {
    Postgres,
    Mongo,
}

#[derive(Debug, Clone, EnumVariantNames, EnumString)]
pub enum TestFramework {
    Jest,
    Vitest,
    PHPUnit,
    Pest,
    Playwright,
    Dusk,
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

fn get_cms() -> bool {
    Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Would you like to use a CMS?")
        .items(&["Yes", "No"])
        .interact()
        .expect("Failed to get CMS selection from user")
        == 0
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