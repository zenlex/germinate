mod config;
mod module;

use clap::{Parser, ValueEnum};
use std::path::PathBuf;
use strum::{EnumString, EnumVariantNames};

use crate::config::ScaffoldConfig;

#[derive(Debug, Clone, EnumVariantNames, EnumString, ValueEnum)]
pub enum StackTemplate {
    SSRJS,   // Server Side Rendered JavaScript/TypeScript
    SPAJS,   // Single Page Application JavaScript/TypeScript
    Laravel, // PHP Laravel with Vue + Inertia
    TSCLI,   // TypeScript CLI / Native App
    RSCLI,   // Rust CLI / Native App
    TSAPI,   // TypeScript API (express backend with optional db only)
    RSAPI,   // Rust API (Axum back end with optional db only)
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

    // run scaffolding engine
    //? The idea here - (architect this more on paper/miro first!!) create a StackBuilder struct with associate methods for each step of constructing a stack(builder pattern) - maybe it takes a generic type for the stack?
    //? Maybe the end shape of a StackBuilder is a nested vec of commands to run, where each subvec is a thread, and commands within a vec are run sequentially. It will need methods to create containers, install dependencies/build a manifest, copy/build templates, and make API calls (push to github, etc). Each of those is its own struct / trait object that implements a common interface (APICall, Container, Template). I think the dependencies bit might be easiest by just building a manifest with a templating engine and building a dockerfile that runs the package installer.
    //? For now we can hard code a module for each stack that instantiates a StackBuilder by calling its methods in order
    //? Scafoolding engine then passes config to builder which grabs trait object for specific stack builder? or maybe just a generic builder that takes a stack type? want to get all the steps for a given stack or stack component into its own module so you can just plug in another builder module stack and have it work
    //?  Can we parallelize it? If the build components are in separate modules, we can potentially cue them up with a thread pool and run them in parallel but would need a way to track dependencies between components the more we can isolate the platforms the more this becomes possible. Get the stack builder to work single threaded with as much composition as makes any sense with a mind towards being able to parallelize it later
    //? then down the road we can add a module to generate a stack builder from a config file

    // return success/errors
    //? Collect success from the scaffold engine and return it to the user
}
