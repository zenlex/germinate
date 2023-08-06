mod builder;
#[allow(warnings)]
mod config;
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
            Self::SSRJS => PathBuf::from("templates/ssr-js.toml"),
            Self::SPAJS => PathBuf::from("templates/spa-js.toml"),
            Self::Laravel => PathBuf::from("templates/laravel.toml"),
            Self::TSCLI => PathBuf::from("templates/ts-cli.toml"),
            Self::RSCLI => PathBuf::from("templates/rs-cli.toml"),
            Self::TSAPI => PathBuf::from("templates/ts-api.toml"),
            Self::RSAPI => PathBuf::from("templates/rs-api.toml"),
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
    // TODO: parse command line args (need to make them optional in a logical way and decide on precedence)
    let args = ScaffoldArgs::parse();
    println!("{:?}", args);

    // get user options
    let user_config = config::get_user_config().unwrap();

    // create config
    let app_config = ScaffoldConfig::new(user_config);
    println!("->> APP_CONFIG: {:?}", app_config);

    let builder = ProjectBuilder::new(app_config);
    builder.build();
    // run scaffolding engine
    //? Create a Builder for the App Config, and then we may also need sub builders for depencies and such
    //? Builder should parse the toml file for the stack to populate the config and then: create folders, install dependencies, template dockerfile, build docker image, create commit and push to git repo
    //?  Can we parallelize it? (future optimization, but keep thinks modularized with a mind towards this end)

    // return success/errors
    //? Collect success from the scaffold engine and return it to the user
}
