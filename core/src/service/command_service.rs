use crate::infrastructure::{Environment, Error};
use crate::model::Content;
use crate::repository::{
    ContentRepository, LocalContentRepository, LocalSiteRepository,
    SiteRepository,
};
use crate::service::content_service::ContentService;
use crate::service::site_service::SiteService;
use crate::template::{DefaultRenderer, Renderer};
use itertools::Itertools;
use serde_json::Value;
use std::iter::FromIterator;
use std::path::Path;

type Result<T> = std::result::Result<T, Error>;

pub struct CommandService<'a> {
    environment: &'a Environment,
    site_service: SiteService<'a>,
    content_service: ContentService<'a>,
}

impl<'a> CommandService<'a> {
    pub fn new(environment: &Environment) -> CommandService {
        let site_service = SiteService::new(environment);
        let content_service = ContentService::new(environment);
        CommandService {
            environment,
            site_service,
            content_service,
        }
    }
    fn create_model(&self) -> Result<Value> {
        let site = self.site_service.load()?;

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
        let mut model = json!({
            "title":site.title,
            "contents":content_groups
        });
        Ok(model)
    }

    pub fn new_site(&self) -> Result<()> {
        self.site_service.create()?;
        Ok(())
    }

    pub fn generate(&self) -> Result<()> {
        let site = self.site_service.load()?;
        let template_path = Path::new(&self.environment.workspace)
            .join(&site.theme_directory)
            .join(&site.theme)
            .join("layout");
        let mut renderer = DefaultRenderer::new();
        renderer.load_templates(&template_path);
        let data = self.create_model()?;
        for template in renderer.get_major_templates() {
            let file_map = renderer.render(&template, &data)?;
            println!("{:?}", file_map);
        }
        Ok(())
    }

    pub fn publish() -> Result<()> {
        unimplemented!()
    }
}
