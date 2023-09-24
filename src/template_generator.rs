use std::{
    fs::{self, File},
    path::PathBuf,
};

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

pub trait TemplateData {}

pub fn generate_dir<T>(src: PathBuf, dest: PathBuf, data: &T, recursive: bool)
where
    T: TemplateData + Serialize,
{
    fs::create_dir_all(&dest).expect("Failed to create directory");
    for file in fs::read_dir(src).unwrap() {
        if let Ok(file) = file {
            if file.file_type().unwrap().is_dir() && recursive {
                println!("Generating directory: {:?}", file.file_name());
                let new_dest = dest.join(file.file_name().into_string().unwrap());
                generate_dir(file.path(), new_dest, data, recursive);
            } else {
                println!("Generating file: {:?}", dest.join(file.file_name()));
                let template = fs::read_to_string(file.path()).expect("Failed to read template");
                let new_file = dest.join(file.file_name());
                crate::template_generator::render_to_file(
                    &template,
                    data,
                    &mut fs::File::create(new_file).unwrap(),
                )
                .expect((format!("Failed to render template: {:?}", &file)).as_str());
            }
        }
    }
}
