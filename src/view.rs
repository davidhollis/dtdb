use axum::response::Html;
use tera::Context;
use tera::Tera;
use include_dir::{include_dir, Dir, DirEntry, File};

static TEMPLATES_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/views");

pub struct Views {
    tera: Tera
}

impl Views {
    pub fn init() -> anyhow::Result<Views> {
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

    pub fn render_page(&self, page: &str) -> anyhow::Result<Html<String>> {
        let template_name = format!("pages/{}.tera.html", page);
        Ok(Html(self.tera.render(template_name.as_str(), &Context::new())?))
    }

    pub fn render_page_with(&self, page: &str, context: &tera::Context) -> anyhow::Result<Html<String>> {
        let template_name = format!("pages/{}.tera.html", page);
        Ok(Html(self.tera.render(template_name.as_str(), context)?))
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