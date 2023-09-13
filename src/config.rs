use std::{collections::HashMap, path::PathBuf, vec};

use crate::{
    db_client::DbClient,
    dialogue::StackTemplate,
    dialogue::{Database, UserOptions},
    linters::Linter,
    module::Module,
    toml_parser::TomlTemplate,
};

type NpmDeps = Vec<Module>;
type CargoDeps = Vec<Module>;
pub type PackageScripts = HashMap<String, String>;

#[derive(Debug, Clone, PartialEq)]
pub enum Language {
    Rust,
    JavaScript,
    TypeScript,
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct ScaffoldConfig {
    user_options: UserOptions,
    title: String,
    root_dir: PathBuf,
    languages: Vec<Language>,
    db: Option<Database>,
    db_client: Option<DbClient>,
    linters: Vec<Linter>,
    npm_scripts: Option<PackageScripts>,
    cargo_scripts: Option<PackageScripts>,
    npm_deps: Option<NpmDeps>,
    cargo_deps: Option<CargoDeps>,
    subfolders: Option<Vec<PathBuf>>,
    containers: bool,
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
        let cargo_scripts = scripts.get("cargo").cloned();

        let npm_deps = dependencies.get("npm").unwrap().clone();
        let cargo_deps = dependencies.get("cargo").unwrap().clone();

        let db = options.db.clone();

        let db_client = match &db {
            Some(db_platform) => match db_platform {
                Database::Postgres => match options.stack {
                    StackTemplate::RSWEB | StackTemplate::RSCLI => match options.orm {
                        true => Some(DbClient::Diesel),
                        false => Some(DbClient::Sqlx),
                    },
                    _ => match options.orm {
                        true => Some(DbClient::Prisma),
                        false => Some(DbClient::Slonik),
                    },
                },
                Database::Sqlite => match options.stack {
                    StackTemplate::RSWEB | StackTemplate::RSCLI => match options.orm {
                        true => Some(DbClient::Diesel),
                        false => Some(DbClient::Sqlx),
                    },
                    _ => match options.orm {
                        true => Some(DbClient::Prisma),
                        false => Some(DbClient::BetterSqlite),
                    },
                },
                Database::Mongo => match options.stack {
                    StackTemplate::RSWEB | StackTemplate::RSCLI => match options.orm {
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
            StackTemplate::RSWEB => vec![Language::Rust],
            StackTemplate::RSCLI => vec![Language::Rust],
            _ => vec![Language::TypeScript, Language::JavaScript],
        };

        let linters = match options.stack {
            StackTemplate::TSWEB => vec![Linter::ESLint, Linter::Stylelint],
            StackTemplate::TSAPI | StackTemplate::TSCLI => vec![Linter::ESLint],
            StackTemplate::RSCLI | StackTemplate::RSWEB => {
                vec![Linter::Clippy]
            }
        };

        Self {
            title,
            root_dir,
            languages,
            db,
            db_client,
            linters,
            npm_scripts,
            cargo_scripts,
            npm_deps,
            cargo_deps,
            subfolders,
            containers: options.containers,
            user_options: options,
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

    pub fn get_npm_scripts(&self) -> &Option<PackageScripts> {
        &self.npm_scripts
    }

    // TODO? May need this later for custom build scripts
    // pub fn get_cargo_scripts(&self) -> &Option<PackageScripts> {
    //     &self.cargo_scripts
    // }

    pub fn get_linters(&self) -> &Vec<Linter> {
        &self.linters
    }

    pub fn get_database(&self) -> &Option<Database> {
        &self.db
    }

    pub fn get_db_client(&self) -> &Option<DbClient> {
        &self.db_client
    }

    pub fn has_language(&self, language: &Language) -> bool {
        self.languages.contains(language)
    }

    pub fn get_stack(&self) -> &StackTemplate {
        &self.user_options.stack
    }

    pub fn get_spa(&self) -> bool {
        self.user_options.spa
    }

    pub fn get_template_engine(&self) -> bool {
        self.user_options.template_engine
    }
}
