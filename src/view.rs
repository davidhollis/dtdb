use std::{error::Error, fmt::Display};

use axum::response::{Html, IntoResponse};
use hyper::StatusCode;
use serde::Serialize;
use tera::Context;
use tera::Tera;
use tera::Error as TeraError;
use include_dir::{include_dir, Dir, DirEntry, File};

static TEMPLATES_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/views");

pub struct Views {
    tera: Tera
}

impl Views {
    pub fn init() -> Result<Views, ViewError> {
        // Create a new empty Tera instance
        let mut tera = Tera::default();

        // Then load in all the static templates
        let mut static_templates = vec!{};
        extract_templates_from(&TEMPLATES_DIR, &mut static_templates);
        tera.add_raw_templates(static_templates)?;
        tera.build_inheritance_chains()?;
        tera.check_macro_files()?;

        log::debug!("Successfully loaded templates: {:?}", tera.get_template_names().collect::<Vec<&str>>());

        Ok(Views {tera})
    }

    pub fn render_page(&self, page: &str, context: impl Serialize) -> Result<Html<String>, ViewError> {
        let template_name = format!("pages/{}.tera.html", page);
        let context = Context::from_serialize(context)?;
        Ok(Html(self.tera.render(template_name.as_str(), &context)?))
    }
}

#[derive(Serialize)]
pub struct EmptyContext {}

#[derive(Debug)]
pub struct ViewError {
    caused_by: TeraError
}

impl Error for ViewError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.caused_by)
    }
}

impl Display for ViewError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The following error occured while rendering views: {}", self.caused_by)
    }
}

impl From<TeraError> for ViewError {
    fn from(err: TeraError) -> Self {
        ViewError { caused_by: err }
    }
}

impl IntoResponse for ViewError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}

fn get_file_details(file: &File) -> Option<(String, String)> {
    file.path().to_str().and_then(|path|
        file.contents_utf8().map(|contents|
            (path.to_string(), contents.to_string())
        )
    )
}

fn extract_templates_from(dir: &Dir, into: &mut Vec<(String, String)>) {
    for entry in dir.entries() {
        match entry {
            DirEntry::Dir(subdir) => extract_templates_from(subdir, into),
            DirEntry::File(file) => {
                if let Some(file_details) = get_file_details(file) {
                    log::debug!("Found template file {}", file_details.0);
                    into.push(file_details)
                }
            }
        }
    }
}