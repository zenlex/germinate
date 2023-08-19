mod builder;
#[allow(warnings)]
mod config;
mod dialogue;
mod module;
mod toml_parser;

use clap::{Parser, ValueEnum};
use std::path::PathBuf;
use strum::{EnumIter, EnumProperty, EnumString, EnumVariantNames};

use crate::{builder::ProjectBuilder, config::ScaffoldConfig};

#[derive(Debug, Clone, EnumVariantNames, EnumString, ValueEnum, EnumIter, EnumProperty)]
pub enum StackTemplate {
    #[strum(props(Label = "SSR JavaScript/TypeScript"))]
    SSRJS,
    #[strum(props(Label = "SPA JavaScript/TypeScript"))]
    SPAJS,
    #[strum(props(Label = "Laravel with Vue + Inertia"))]
    Laravel,
    #[strum(props(Label = "TypeScript CLI Tool"))]
    TSCLI,
    #[strum(props(Label = "Rust CLI Tool"))]
    RSCLI,
    #[strum(props(Label = "TypeScript API (backend only)"))]
    TSAPI,
    #[strum(props(Label = "Rust API (backend only)"))]
    RSAPI,
}

impl StackTemplate {
    pub fn get_path(&self) -> PathBuf {
        match self {
            Self::SSRJS => PathBuf::from("templates/ssrjs/stack_template.toml"),
            Self::SPAJS => PathBuf::from("templates/spajs/stack_template.toml"),
            Self::Laravel => PathBuf::from("templates/laravel/stack_template.toml"),
            Self::TSCLI => PathBuf::from("templates/tscli/stack_template.toml"),
            Self::RSCLI => PathBuf::from("templates/rscli/stack_template.toml"),
            Self::TSAPI => PathBuf::from("templates/tsapi/stack_template.toml"),
            Self::RSAPI => PathBuf::from("templates/rsapi/stack_template.toml"),
        }
    }
}
#[derive(Debug, Parser)]
#[command(author, version, about, long_about= None)]
pub struct ScaffoldArgs {
    #[arg(short, long)]
    stack: Option<StackTemplate>,
    #[arg(short, long)]
    output_dir: Option<PathBuf>,
}
fn main() {
    let user_config = dialogue::get_user_config().unwrap();
    let app_config = ScaffoldConfig::new(user_config);
    let builder = ProjectBuilder::new(app_config);
    builder.build();
    //?  Can we parallelize it? (future optimization, but keep thinks modularized with a mind towards this end)

    // return success/errors
    //? Collect success from the scaffold engine and return it to the user
}
