use crate::infrastructure::{Environment, Error};
use crate::model::Site;
use crate::repository::DefaultThemeRepository;
use crate::repository::ThemeRepository;
use crate::repository::{LocalSiteRepository, SiteRepository};
use crate::template::{DefaultRenderer, Renderer};
use crate::ContentService;
use serde_json::Value;
use std::fs::DirBuilder;
use std::fs::File;
use std::io::Write;
use std::path::Path;

type Result<T> = std::result::Result<T, Error>;

pub struct SiteService<'a> {
    content_service: ContentService<'a>,
    site_repository: Box<dyn SiteRepository + 'a>,
}

impl<'a> SiteService<'a> {
    fn create_model(&self) -> Result<Value> {
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
            site_repository,
            content_service,
        }
    }

    pub fn create(&self) -> Result<Site> {
        self.site_repository.create().map(|x| Site::from(x))
    }

    pub fn load(&self) -> Result<Site> {
        self.site_repository.load().map(|x| Site::from(x))
    }

    pub fn generate(&self) -> Result<()> {
        let site = self.load()?;
        let mut renderer = DefaultRenderer::new();
        let theme_repository: Box<dyn ThemeRepository> =
            Box::new(DefaultThemeRepository::new(&site.theme_directory));
        let templates = theme_repository.templates();
        for name in templates {
            renderer
                .register_template_string(
                    &name,
                    &theme_repository.template(&name).unwrap(),
                )
                .unwrap();
        }
        let data = self.create_model()?;
        for template in renderer.get_major_templates() {
            let file_map = renderer.render(&template, &data)?;
            for (name, content) in &file_map {
                let file_path = Path::new(&site.root)
                    .join(&site.build_directory)
                    .join(name);
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

        let assets_path = Path::new(&site.root).join(&site.build_directory);
        theme_repository.save_assets(&assets_path).unwrap();
        Ok(())
    }

    pub fn publish(&self) -> Result<()> {
        unimplemented!()
    }
}
