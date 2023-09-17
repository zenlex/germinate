use std::fs::File;

use handlebars::{Handlebars, RenderError};
use serde::Serialize;

pub fn render_to_file<T>(template: &str, data: &T, file: &mut File) -> Result<(), RenderError>
where
    T: Serialize,
{
    let handlebars = Handlebars::new();
    handlebars.render_template_to_write(template, data, file)?;
    Ok(())
}
