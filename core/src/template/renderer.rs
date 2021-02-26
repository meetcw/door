use crate::infrastructure::Error;
use crate::template::helper::*;
use handlebars::*;
use serde::Serialize;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use std::sync::RwLock;

type Result<T> = std::result::Result<T, Error>;

pub trait Renderer {
    fn new() -> Self;
    fn register_layout_string(
        &mut self,
        name: &str,
        template: &str,
    ) -> Result<()>;
    fn load_layouts(&mut self, path: &Path) -> Result<()>;
    fn get_layouts(&self) -> Vec<String>;
    fn get_major_layouts(&self) -> Vec<String>;
    fn get_component_layouts(&self) -> Vec<String>;
    fn render<T>(
        &self,
        name: &str,
        data: &T,
    ) -> Result<HashMap<String, String>>
    where
        T: Serialize;

    fn render_string<T>(&self, layout: &str, data: &T) -> Result<String>
    where
        T: Serialize;
}

pub struct DefaultRenderer<'a> {
    handlebars: Handlebars<'a>,
    file_map: Arc<RwLock<HashMap<String, String>>>,
}

impl<'a> DefaultRenderer<'a> {}

impl<'a> Renderer for DefaultRenderer<'a> {
    fn new() -> Self {
        let handlebars = Handlebars::new();

        let file_map = Arc::new(RwLock::new(HashMap::new()));
        let file_helper = Box::new(FileHelper {
            file_map: Arc::downgrade(&file_map),
        });

        let mut renderer = DefaultRenderer {
            handlebars,
            file_map,
        };
        renderer
            .handlebars
            .register_helper("group", Box::new(GroupHelper {}));
        renderer
            .handlebars
            .register_helper("sort", Box::new(SortHelper {}));
        renderer.handlebars.register_helper("file", file_helper);
        renderer
            .handlebars
            .register_helper("json", Box::new(JsonHelper {}));
        renderer
            .handlebars
            .register_helper("concat", Box::new(ConcatHelper {}));
        renderer
            .handlebars
            .register_helper("markdown", Box::new(MarkdownHelper {}));
        renderer
            .handlebars
            .register_helper("markdown-toc", Box::new(MarkdownTOCHelper {}));
        renderer
            .handlebars
            .register_helper("datetime", Box::new(DatetimeHelper {}));
        renderer
            .handlebars
            .register_helper("assign", Box::new(AssignHelper {}));
        return renderer;
    }

    fn register_layout_string(
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

    fn load_layouts(&mut self, _path: &Path) -> Result<()> {
        // return self
        //     .handlebars
        //     .register_templates_directory(".hbs", path)
        //     .map_err(|error| {
        //         Error::new("Failed to register templates.")
        //             .with_inner_error(&error)
        //     });
        todo!()
    }

    fn get_layouts(&self) -> Vec<String> {
        let mut keys = vec![];
        let template_map = self.handlebars.get_templates();
        for key in template_map.keys() {
            keys.push(key.to_string());
        }
        return keys;
    }

    fn get_major_layouts(&self) -> Vec<String> {
        let mut keys = vec![];
        let template_map = self.handlebars.get_templates();
        for key in template_map.keys() {
            if !key.contains("_") {
                keys.push(key.to_string());
            }
        }
        return keys;
    }

    fn get_component_layouts(&self) -> Vec<String> {
        let mut keys = vec![];
        let template_map = self.handlebars.get_templates();
        for key in template_map.keys() {
            if key.contains("_") {
                keys.push(key.to_string());
            }
        }
        return keys;
    }
    fn render<T>(&self, name: &str, data: &T) -> Result<HashMap<String, String>>
    where
        T: Serialize,
    {
        self.handlebars.render(name, data).unwrap();
        let mut writeable_file_map = self.file_map.write().unwrap();
        let file_map = writeable_file_map.clone();
        writeable_file_map.clear();
        Ok(file_map)
    }

    fn render_string<T>(&self, template: &str, data: &T) -> Result<String>
    where
        T: Serialize,
    {
        let result = self.handlebars.render_template(template, data).unwrap();
        Ok(result)
    }
}
