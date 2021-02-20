use crate::infrastructure::{Environment, Error};
use crate::model::{Content, Site};
use crate::repository::DefaultThemeRepository;
use crate::repository::ThemeRepository;
use crate::template::{DefaultRenderer, Renderer};
use crate::ContentService;
use serde_json::Value;
use itertools::Itertools;
use std::iter::FromIterator;

type Result<T> = std::result::Result<T, Error>;
use crate::repository::{
    ContentRepository, LocalContentRepository, LocalSiteRepository,
    SiteRepository,
};
use std::cmp::Ordering;

pub struct SiteService<'a> {
    content_service: ContentService<'a>,
    site_repository: Box<dyn SiteRepository + 'a>,
}

impl<'a> SiteService<'a> {
    fn create_model(&self) -> Result<Value> {
        let site = self.load()?;

        let contents = self.content_service.search(
            |x| x.draft == false,
            |a, b| a.create_time.cmp(&b.create_time),
        )?;
        let mut content_groups = vec![];
        content_groups.push(json!({
            "target":"all",
            "contents":contents
        }));
        for (key, items) in &contents.iter().group_by(|x| x.target.clone()) {
            let items = Vec::from_iter(items);
            let content_group = json!({
                "target":key,
                "contents":items
            });
            content_groups.push(content_group);
        }
        let model = json!({
            "title":site.title,
            "contents":content_groups
        });
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
            println!("{:?}", file_map);
        }
        Ok(())
    }

    pub fn publish(&self) -> Result<()> {
        unimplemented!()
    }
}
