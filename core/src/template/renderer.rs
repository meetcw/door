use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::Weak;

use handlebars::*;
use serde_json::Value;

use crate::infrastructure::{utilities, Error};

use super::FileHelper;
use crate::template::helper_json::JsonHelper;

type Result<T> = std::result::Result<T, Error>;

pub trait Renderer {
    fn new() -> Self;
    fn register_template_string(
        &mut self,
        name: &str,
        template: &str,
    ) -> Result<()>;
    fn load_templates(&mut self, path: &Path) -> Result<()>;
    fn get_templates(&self) -> Vec<String>;
    fn get_major_templates(&self) -> Vec<String>;
    fn get_component_templates(&self) -> Vec<String>;
    fn render(
        &self,
        name: &str,
        data: &Value,
    ) -> Result<HashMap<String, String>>;

    fn render_template(&self, template: &str, data: &Value) -> Result<String>;
}

pub struct DefaultRenderer {
    handlebars: Handlebars,
    file_map: Arc<RwLock<HashMap<String, String>>>,
}

impl DefaultRenderer {}

impl Renderer for DefaultRenderer {
    fn new() -> Self {
        let file_map = Arc::new(RwLock::new(HashMap::new()));
        let handlebars = Handlebars::new();

        let file_helper = Box::new(FileHelper {
            file_map: Arc::downgrade(&file_map),
        });
        let mut renderer = DefaultRenderer {
            handlebars,
            file_map,
        };
        renderer.handlebars.register_helper("file", file_helper);
        renderer
            .handlebars
            .register_helper("json", Box::new(JsonHelper {}));
        return renderer;
    }

    fn register_template_string(
        &mut self,
        name: &str,
        template: &str,
    ) -> Result<()> {
        return self
            .handlebars
            .register_template_string(name, template)
            .map_err(|error| {
                Error::new("Failed to register template.")
                    .with_inner_error(&error)
            });
    }

    fn load_templates(&mut self, path: &Path) -> Result<()> {
        return self
            .handlebars
            .register_templates_directory(".hbs", path)
            .map_err(|error| {
                Error::new("Failed to register templates.")
                    .with_inner_error(&error)
            });
    }

    fn get_templates(&self) -> Vec<String> {
        let mut keys = vec![];
        let template_map = self.handlebars.get_templates();
        for key in template_map.keys() {
            keys.push(key.to_string());
        }
        return keys;
    }

    fn get_major_templates(&self) -> Vec<String> {
        let mut keys = vec![];
        let template_map = self.handlebars.get_templates();
        for key in template_map.keys() {
            if !key.contains("_") {
                keys.push(key.to_string());
            }
        }
        return keys;
    }

    fn get_component_templates(&self) -> Vec<String> {
        let mut keys = vec![];
        let template_map = self.handlebars.get_templates();
        for key in template_map.keys() {
            if key.contains("_") {
                keys.push(key.to_string());
            }
        }
        return keys;
    }
    fn render(
        &self,
        name: &str,
        data: &Value,
    ) -> Result<HashMap<String, String>> {
        self.handlebars.render(name, data).unwrap();
        let mut writeable_file_map = self.file_map.write().unwrap();
        let file_map = writeable_file_map.clone();
        writeable_file_map.clear();
        Ok(file_map)
    }

    fn render_template(&self, template: &str, data: &Value) -> Result<String> {
        let result = self.handlebars.render_template(template, data).unwrap();
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use tester::Tester;

    use super::*;

    #[test]
    fn file_helper() {
        Tester::new().run(|| {
            let tpl = "{{#file \"./post/\" name \".html\"}}{{name}}{{/file}}";
            let data = json!({ "name" : "about" });
            let mut renderer = DefaultRenderer::new();
            renderer.register_template_string("about", tpl);
            let file_map = renderer.render("about", &data).unwrap();
            println!("{:?}", file_map);
            assert_eq!(file_map.len(), 1);
        });
    }
}
