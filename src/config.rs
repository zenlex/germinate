use dialoguer::{theme::ColorfulTheme, Input, MultiSelect, Select};
use std::{
    collections::HashMap,
    error::Error,
    path::{Path, PathBuf},
    str::FromStr,
    vec,
};

use slug::slugify;
use strum::{EnumProperty, EnumString, EnumVariantNames, IntoEnumIterator, VariantNames};

use crate::{
    dialogue::{Database, TestFramework, UserOptions},
    module::Module,
    toml_parser::TomlTemplate,
    StackTemplate,
};

type NpmDeps = Vec<Module>;
type CargoDeps = Vec<Module>;
type ComposerDeps = Vec<Module>;
pub type PackageScripts = HashMap<String, String>;

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
enum WebFramework {
    Axum,
    Express,
    Astro,
    Laravel,
    Vue,
    Dioxus,
}

#[derive(Debug, Clone)]
enum DbClient {
    Diesel, // Rust ORM
    Sqlx,   // Rust typed SQL
    Prisma, // TS ORM
    Slonik, // TS typed SQL
}

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
                    StackTemplate::SSRJS => match options.orm {
                        true => Some(DbClient::Prisma),
                        false => Some(DbClient::Slonik),
                    },
                    _ => None,
                },
                _ => None,
            },
            None => None,
        };

        let languages = match options.stack {
            StackTemplate::Laravel => {
                vec![Language::PHP, Language::TypeScript, Language::JavaScript]
            }
            StackTemplate::RSAPI => vec![Language::Rust],
            StackTemplate::RSAPI => vec![Language::Rust],
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
            StackTemplate::SSRJS => vec![Linter::ESLint],
            StackTemplate::SPAJS => vec![Linter::ESLint],
            StackTemplate::Laravel => vec![Linter::ESLint, Linter::Stylelint],
            StackTemplate::TSAPI => vec![Linter::ESLint],
            _ => vec![],
        };

        let formatters = match options.stack {
            StackTemplate::SSRJS => vec![Formatter::ESLint],
            StackTemplate::SPAJS => vec![Formatter::ESLint],
            StackTemplate::Laravel => vec![Formatter::ESLint, Formatter::Pint],
            StackTemplate::TSAPI => vec![Formatter::ESLint],
            _ => vec![],
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

    pub fn get_cargo_scripts(&self) -> &Option<PackageScripts> {
        &self.cargo_scripts
    }

    pub fn get_composer_scripts(&self) -> &Option<PackageScripts> {
        &self.composer_scripts
    }
}
