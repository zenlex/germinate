mod builder;
mod config;
mod db_client;
mod dialogue;
mod formatters;
mod linters;
mod module;
mod test_frameworks;
mod toml_parser;

use crate::{builder::ProjectBuilder, config::ScaffoldConfig};

fn main() {
    let user_config = dialogue::get_user_config().unwrap();
    let app_config = ScaffoldConfig::new(user_config);
    let builder = ProjectBuilder::new(app_config);
    builder.build();
    //?  Can we parallelize it? (future optimization, but keep thinks modularized with a mind towards this end)

    // return success/errors
    //? Collect success from the scaffold engine and return it to the user
}
