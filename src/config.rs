use std::{collections::HashMap, path::PathBuf, vec};

use crate::{
    dialogue::StackTemplate,
    dialogue::{Database, TestFramework, UserOptions},
    linters::Linter,
    module::Module,
    toml_parser::TomlTemplate,
};

type NpmDeps = Vec<Module>;
type CargoDeps = Vec<Module>;
type ComposerDeps = Vec<Module>;
pub type PackageScripts = HashMap<String, String>;

#[derive(Debug, Clone)]
enum Formatter {
    ESLint,
    Pint,
    Rustfmt,
}

#[derive(Debug, Clone)]
enum CMS {
    Filament,
    Strapi,
}

#[derive(Debug, Clone)]
enum Language {
    Rust,
    JavaScript,
    TypeScript,
    PHP,
}

#[derive(Debug, Clone)]
enum DbClient {
    Diesel,       // Rust ORM
    Sqlx,         // Rust typed SQL
    Prisma,       // TS ORM
    Slonik,       // TS typed SQL
    BetterSqlite, // Node SQLite3 driver
    MongoDb,      // Node/Rust/PHP MongoDB driver
    Mongoose,     // Node MongoDB ORM
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct ScaffoldConfig {
    title: String,
    root_dir: PathBuf,
    languages: Vec<Language>,
    test_frameworks: Vec<TestFramework>,
    db: Option<Database>,
    db_client: Option<DbClient>,
    cms: Option<CMS>,
    linters: Vec<Linter>,
    formatters: Vec<Formatter>,
    npm_scripts: Option<PackageScripts>,
    composer_scripts: Option<PackageScripts>,
    cargo_scripts: Option<PackageScripts>,
    npm_deps: Option<NpmDeps>,
    composer_deps: Option<ComposerDeps>,
    cargo_deps: Option<CargoDeps>,
    subfolders: Option<Vec<PathBuf>>,
}

impl ScaffoldConfig {
    pub fn new(options: UserOptions) -> Self {
        let title = options.app_name.clone();
        let root_dir = PathBuf::from(&options.output_dir);
        let toml = TomlTemplate::new(&options.stack.get_path());
        let subfolders = toml.get_subfolders().cloned();
        let dependencies = toml.get_dependencies();
        let scripts = match toml.get_scripts() {
            Some(scripts) => scripts.to_owned(),
            None => HashMap::new(),
        };

        let npm_scripts = scripts.get("npm").cloned();
        let composer_scripts = scripts.get("composer").cloned();
        let cargo_scripts = scripts.get("cargo").cloned();

        let npm_deps = dependencies.get("npm").unwrap().clone();
        let composer_deps = dependencies.get("composer").unwrap().clone();
        let cargo_deps = dependencies.get("cargo").unwrap().clone();

        let db = options.db.clone();

        let db_client = match &db {
            Some(db_platform) => match db_platform {
                Database::Postgres => match options.stack {
                    StackTemplate::Laravel => None,
                    StackTemplate::RSAPI | StackTemplate::RSCLI => match options.orm {
                        true => Some(DbClient::Diesel),
                        false => Some(DbClient::Sqlx),
                    },
                    _ => match options.orm {
                        true => Some(DbClient::Prisma),
                        false => Some(DbClient::Slonik),
                    },
                },
                Database::Sqlite => match options.stack {
                    StackTemplate::Laravel => None,
                    StackTemplate::RSAPI | StackTemplate::RSCLI => match options.orm {
                        true => Some(DbClient::Diesel),
                        false => Some(DbClient::Sqlx),
                    },
                    _ => match options.orm {
                        true => Some(DbClient::Prisma),
                        false => Some(DbClient::BetterSqlite),
                    },
                },
                Database::Mongo => match options.stack {
                    StackTemplate::Laravel => Some(DbClient::MongoDb),
                    StackTemplate::RSAPI | StackTemplate::RSCLI => match options.orm {
                        true => panic!("No Rust ORM for MongoDB"),
                        false => Some(DbClient::MongoDb),
                    },
                    _ => match options.orm {
                        true => Some(DbClient::Mongoose),
                        false => Some(DbClient::MongoDb),
                    },
                },
            },
            None => None,
        };

        let languages = match options.stack {
            StackTemplate::Laravel => {
                vec![Language::PHP, Language::TypeScript, Language::JavaScript]
            }
            StackTemplate::RSAPI => vec![Language::Rust],
            StackTemplate::RSCLI => vec![Language::Rust],
            _ => vec![Language::TypeScript, Language::JavaScript],
        };

        let test_frameworks = options.test_frameworks.clone();

        let cms = match options.cms {
            true => match options.stack {
                StackTemplate::Laravel => Some(CMS::Filament),
                _ => Some(CMS::Strapi),
            },
            false => None,
        };

        let linters = match options.stack {
            StackTemplate::Laravel => vec![Linter::ESLint, Linter::Stylelint, Linter::Larastan],
            StackTemplate::RSAPI | StackTemplate::RSCLI => vec![Linter::Clippy],
            _ => vec![Linter::ESLint, Linter::Stylelint],
        };

        let formatters = match options.stack {
            StackTemplate::Laravel => vec![Formatter::ESLint, Formatter::Pint],
            StackTemplate::RSAPI | StackTemplate::RSCLI => vec![Formatter::Rustfmt],
            _ => vec![Formatter::ESLint],
        };

        Self {
            title,
            root_dir,
            languages,
            test_frameworks,
            db,
            db_client,
            cms,
            linters,
            formatters,
            npm_scripts,
            composer_scripts,
            cargo_scripts,
            npm_deps,
            composer_deps,
            cargo_deps,
            subfolders,
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

    pub fn get_npm_scripts(&self) -> &Option<PackageScripts> {
        &self.npm_scripts
    }

    // TODO? May need this later for custom build scripts
    // pub fn get_cargo_scripts(&self) -> &Option<PackageScripts> {
    //     &self.cargo_scripts
    // }

    pub fn get_composer_scripts(&self) -> &Option<PackageScripts> {
        &self.composer_scripts
    }

    pub fn get_linters(&self) -> &Vec<Linter> {
        &self.linters
    }
}
