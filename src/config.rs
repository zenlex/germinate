use std::{collections::HashMap, env, path::PathBuf, vec};

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
    pub user_options: UserOptions,
    pub title: String,
    pub root_dir: PathBuf,
    pub template_dir: PathBuf,
    pub languages: Vec<Language>,
    pub db: Option<Database>,
    pub db_client: Option<DbClient>,
    pub linters: Vec<Linter>,
    pub npm_scripts: Option<PackageScripts>,
    pub cargo_scripts: Option<PackageScripts>,
    pub npm_deps: Option<NpmDeps>,
    pub cargo_deps: Option<CargoDeps>,
    pub subfolders: Option<Vec<PathBuf>>,
    pub containers: bool,
}

impl ScaffoldConfig {
    pub fn new(options: UserOptions) -> Self {
        let title = options.app_name.clone();
        let root_dir = PathBuf::from(&options.output_dir);
        let template_dir = env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .join(options.stack.get_path().parent().unwrap());

        let toml = TomlTemplate::new(&template_dir.join("stack_template.toml"));
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
            StackTemplate::RSAPI => vec![Language::Rust],
            StackTemplate::RSCLI => vec![Language::Rust],
            _ => vec![Language::TypeScript, Language::JavaScript],
        };

        let linters = match options.stack {
            StackTemplate::TSCLI => vec![Linter::ESLint],
            StackTemplate::TSAPI => match options.spa {
                true => {
                    vec![Linter::ESLint, Linter::Stylelint]
                }
                false => {
                    vec![Linter::ESLint]
                }
            },
            StackTemplate::RSCLI | StackTemplate::RSAPI => {
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
            user_options: options.clone(),
            template_dir,
        }
    }

    pub fn has_language(&self, language: &Language) -> bool {
        self.languages.contains(language)
    }
}
