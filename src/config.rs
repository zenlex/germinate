use std::{error::Error, path::PathBuf};

use clap::Parser;

use crate::StackTemplate;

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
enum Database {
    Postgres,
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
    Prettier,
    PhpCsFixer,
    Pint,
    Rustfmt,
}

pub struct ScaffoldConfig {
    languages: Vec<Language>,
    web_frameworks: Vec<WebFramework>,
    test_frameworks: Vec<TestFramework>,
    db: Option<Database>,
    db_client: Option<DbClient>,
    cms: Option<CMS>,
    linters: Vec<Linter>,
    formatters: Vec<Formatter>,
    dependencies: Vec<Box<dyn BuildDep>>,
}

impl ScaffoldConfig {
    // TODO: get an object of command line args and parse them into a ScaffoldConfig
    pub fn new(stack: StackTemplate) -> Self {
        match stack {
            StackTemplate::RSCLI => Self {
                languages: vec![Language::Rust],
                web_frameworks: vec![],
                test_frameworks: vec![],
                db: None,
                db_client: None,
                cms: None,
                linters: vec![],
                formatters: vec![],
                dependencies: vec![Box::new(Dependency::Cargo {
                    name: "cargo-make".to_string(),
                    version: "0.33.1".to_string(),
                    features: vec![],
                })],
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
                dependencies: vec![],
            },
        }
    }

    pub fn add_language(&mut self, language: Language) {
        self.languages.push(language);
    }

    pub fn add_web_framework(&mut self, web_framework: WebFramework) {
        self.web_frameworks.push(web_framework);
    }

    pub fn add_test_framework(&mut self, test_framework: TestFramework) {
        self.test_frameworks.push(test_framework);
    }

    pub fn set_db(&mut self, db: Database) {
        self.db = Some(db);
    }

    pub fn set_db_client(&mut self, db_client: DbClient) {
        self.db_client = Some(db_client);
    }

    pub fn set_cms(&mut self, cms: CMS) {
        self.cms = Some(cms);
    }

    pub fn add_linter(&mut self, linter: Linter) {
        self.linters.push(linter);
    }

    pub fn add_formatter(&mut self, formatter: Formatter) {
        self.formatters.push(formatter);
    }
}

trait BuildDep {
    fn build(&self) -> Result<(), Box<dyn Error>>;
    fn destroy(&self) -> Result<(), Box<dyn Error>>;
}

enum Dependency {
    Npm {
        name: String,
        version: String,
    },
    Cargo {
        name: String,
        version: String,
        features: Vec<String>,
    },
    Composer {
        name: String,
        version: String,
    },
    Yarn {
        name: String,
        version: String,
    },
}

impl BuildDep for Dependency {
    fn build(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn destroy(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

#[derive(Debug)]
pub struct UserOptions {
    pub stack: StackTemplate,
    pub output_dir: PathBuf,
    pub app_name: String,
}
