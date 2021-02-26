use crate::infrastructure::{Environment, Error};
use crate::model::Site;
use crate::repository::save_default_template;
use crate::repository::LocalTemplateRepository;
use crate::repository::TemplateRepository;
use crate::repository::{LocalSiteRepository, SiteRepository};
use crate::template::{DefaultRenderer, Renderer};
use crate::ContentService;
use serde_json::Value;
use std::fs::DirBuilder;
use std::fs::File;
use std::io::Write;

type Result<T> = std::result::Result<T, Error>;

pub struct SiteService<'a> {
    environment: &'a Environment,
    content_service: ContentService<'a>,
    site_repository: Box<dyn SiteRepository + 'a>,
}

impl<'a> SiteService<'a> {
    fn render_model(&self) -> Result<Value> {
        let site = self.load()?;

        let contents = self
            .content_service
            .search(|_| true, |a, b| a.create_time.cmp(&b.create_time))?;
        let mut model = serde_json::to_value(&site).unwrap();
        model["contents"] = serde_json::to_value(&contents).unwrap();
        debug!(
            "Create model\n{}",
            serde_json::to_string_pretty(&model).unwrap()
        );
        Ok(model)
    }

    pub fn new(environment: &'a Environment) -> SiteService {
        let site_repository = Box::new(LocalSiteRepository::new(environment));
        let content_service = ContentService::new(environment);

        SiteService {
            environment,
            site_repository,
            content_service,
        }
    }

    pub fn create(&self) -> Result<Site> {
        let site = self.site_repository.create().map(|x| Site::from(x))?;
        save_default_template(
            &self.environment.template_directory.join("default"),
        )?;
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
        let site = self.load()?;
        let mut renderer = DefaultRenderer::new();
        let template_repository: Box<dyn TemplateRepository> =
            Box::new(LocalTemplateRepository::new(&self.environment));
        let layouts = template_repository.layouts(&site.template);
        debug!("Find {} render layouts", layouts.len());
        for name in layouts {
            renderer
                .register_layout_string(
                    &name,
                    &template_repository.layout(&site.template, &name).unwrap(),
                )
                .unwrap();
        }
        let data = self.render_model()?;

        for layout in renderer.get_layouts() {
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

        template_repository
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
