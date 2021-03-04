use crate::entity::TemplateEntity;
use crate::infrastructure::*;
use std::fs::{DirBuilder, File};
use std::io::Write;
use std::path::Path;

type Result<T> = std::result::Result<T, Error>;

const TEMPLATE_RESOURCE_PATH: &str = "template";

pub fn save_default_template(path: &Path) -> Result<()> {
    let template_file_paths: Vec<String> =
        Resource::list(TEMPLATE_RESOURCE_PATH);
    for source_file_path in template_file_paths {
        let file_content = Resource::get_content(&source_file_path);
        let source_file_path = Path::new(&source_file_path);
        let target_file_path = path.join(
            source_file_path
                .strip_prefix(TEMPLATE_RESOURCE_PATH)
                .unwrap(),
        );

        DirBuilder::new()
            .recursive(true)
            .create(target_file_path.parent().unwrap())
            .map_err(|error| {
                Error::new(
                    "An error occurred while creating the target directory.",
                )
                .with_inner_error(&error)
            })?;

        let mut file = File::create(target_file_path).unwrap();
        file.write_all(file_content).unwrap();
    }
    return Ok(());
}

pub trait TemplateRepository {
    fn layouts(&self, name: &str) -> Vec<String>;
    fn layout(&self, name: &str, template_name: &str) -> Option<String>;
    fn save_static_files(&self, name: &str, path: &Path) -> Result<()>;
    fn template(&self, name: &str) -> Result<TemplateEntity>;
    fn template_script(&self, name: &str) -> Option<String>;
}

pub struct LocalTemplateRepository<'a> {
    environment: &'a Environment,
}

impl<'a> LocalTemplateRepository<'a> {
    pub fn new(environment: &'a Environment) -> LocalTemplateRepository<'a> {
        LocalTemplateRepository { environment }
    }
}

impl<'a> TemplateRepository for LocalTemplateRepository<'a> {
    fn layouts(&self, name: &str) -> Vec<String> {
        let layout_path = self
            .environment
            .template_directory
            .join(name)
            .join("layout");
        list_files(&layout_path, true, |path| {
            path.extension().unwrap() == "hbs"
        })
        .unwrap()
        .iter()
        .map(|path| {
            path.strip_prefix(&layout_path)
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
        })
        .collect()
    }
    fn layout(&self, name: &str, template_name: &str) -> Option<String> {
        let path = self
            .environment
            .template_directory
            .join(name)
            .join("layout")
            .join(template_name);
        if path.exists() {
            std::fs::read_to_string(&path).ok()
        } else {
            None
        }
    }
    fn save_static_files(&self, name: &str, path: &Path) -> Result<()> {
        let static_path = self
            .environment
            .template_directory
            .join(name)
            .join("static");
        copy_files(&static_path, path, |_| true)
    }
    fn template(&self, name: &str) -> Result<TemplateEntity> {
        let template_path = self
            .environment
            .template_directory
            .join(name)
            .join("template.json");

        if !template_path.exists() {
            return Err(Error::new("The template file is not exists."));
        }

        let template_content = std::fs::read_to_string(&template_path)
            .map_err(|error| {
                Error::new("Failed to read the template file.")
                    .with_inner_error(&error)
            })?;

        let template = serde_json::from_str::<TemplateEntity>(
            &template_content,
        )
        .map_err(|error| {
            Error::new("Failed to resolve the template file.")
                .with_inner_error(&error)
        })?;
        return Ok(template);
    }

    fn template_script(&self, name: &str) -> Option<String> {
        let template_script_path = self
            .environment
            .template_directory
            .join(name)
            .join("template.rhai");

        if !template_script_path.exists() {
            return None;
        }

        std::fs::read_to_string(&template_script_path).ok()
    }
}
