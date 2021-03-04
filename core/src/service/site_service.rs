use crate::infrastructure::{Environment, Error};
use crate::model::Site;
use crate::repository::save_default_template;
use crate::repository::LocalTemplateRepository;
use crate::repository::TemplateRepository;
use crate::repository::{LocalSiteRepository, SiteRepository};
use crate::template::{DefaultRenderer, Renderer};
use crate::ContentService;
use rhai::serde::{from_dynamic, to_dynamic};
use rhai::Dynamic;
use rhai::Engine;
use rhai::Scope;
use serde_json::Value;
use std::fs::DirBuilder;
use std::fs::File;
use std::io::Write;

type Result<T> = std::result::Result<T, Error>;

pub struct SiteService<'a> {
    environment: &'a Environment,
    content_service: ContentService<'a>,
    site_repository: Box<dyn SiteRepository + 'a>,
    template_repository: Box<dyn TemplateRepository + 'a>,
}

impl<'a> SiteService<'a> {
    fn render_model(&self) -> Result<Value> {
        let site = self.load()?;

        let contents = self
            .content_service
            .search(|_| true, |a, b| a.create_time.cmp(&b.create_time))?;
        let mut model = serde_json::to_value(&site).unwrap();
        model["contents"] = serde_json::to_value(&contents).unwrap();

        match self.template_repository.template_script(&site.template) {
            Some(script_content) => {
                let engine = Engine::new();
                let script =
                    engine.compile(&script_content).map_err(|error| {
                        Error::new("Invalid template script.")
                            .with_inner_error(&error)
                    })?;
                let mut scope = Scope::new();
                let script_data = to_dynamic(model.clone()).unwrap();
                scope.push_dynamic("model", script_data);
                let result: Dynamic = engine
                    .eval_ast_with_scope(&mut scope, &script)
                    .map_err(|error| {
                        Error::new("Invalid template script result.")
                            .with_inner_error(&error)
                    })?;

                model = serde_json::to_value(result).unwrap();
            }
            None => (),
        };
        debug!(
            "Create model\n{}",
            serde_json::to_string_pretty(&model).unwrap()
        );
        Ok(model)
    }

    pub fn new(environment: &'a Environment) -> SiteService {
        let site_repository = Box::new(LocalSiteRepository::new(environment));
        let template_repository =
            Box::new(LocalTemplateRepository::new(environment));
        let content_service = ContentService::new(environment);
        SiteService {
            environment,
            site_repository,
            content_service,
            template_repository,
        }
    }

    pub fn create(&self) -> Result<Site> {
        let site = self.site_repository.create().map(|x| Site::from(x))?;
        Ok(site)
    }

    pub fn load(&self) -> Result<Site> {
        self.site_repository.load().map(|x| Site::from(x))
    }

    pub fn clean(&self) -> Result<()> {
        let generate_path = &self.environment.generate_directory;
        if generate_path.exists() {
            std::fs::remove_dir_all(&generate_path).map_err(|error| {
                Error::new("An error occurred while clean generate directory.")
                    .with_inner_error(&error)
            })?;
        }
        Ok(())
    }

    pub fn generate(&self) -> Result<()> {
        self.clean()?;
        let template_path = self.environment.template_directory.join("default");
        if !template_path.exists() {
            save_default_template(
                &self.environment.template_directory.join("default"),
            )?;
        }
        let site = self.load()?;
        let mut renderer = DefaultRenderer::new();
        let layouts = self.template_repository.layouts(&site.template);
        debug!("Find {} render layouts", layouts.len());
        for name in layouts {
            renderer
                .register_layout_string(
                    &name,
                    &self
                        .template_repository
                        .layout(&site.template, &name)
                        .unwrap(),
                )
                .unwrap();
        }
        let data = self.render_model()?;

        for layout in renderer.get_major_layouts() {
            let file_map = renderer.render(&layout, &data)?;
            for (name, content) in &file_map {
                let file_path = self.environment.generate_directory.join(name);
                DirBuilder::new()
                    .recursive(true)
                    .create(file_path.parent().unwrap())
                    .map_err(|error| {
                        Error::new(
                    "An error occurred while creating the target directory.",
                )
                .with_inner_error(&error)
                    })?;
                let mut file = File::create(file_path).unwrap();
                file.write_all(content.as_bytes()).unwrap();
            }
        }

        self.template_repository
            .save_static_files(
                &site.template,
                &self.environment.generate_directory,
            )
            .unwrap();
        Ok(())
    }

    pub fn publish(&self) -> Result<()> {
        unimplemented!()
    }
}
