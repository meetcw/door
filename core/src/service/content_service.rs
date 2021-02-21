use crate::infrastructure::{Environment, Error};
use crate::model::Content;
use crate::repository::{
    ContentRepository, LocalContentRepository, LocalSiteRepository,
    SiteRepository,
};
use std::cmp::Ordering;

type Result<T> = std::result::Result<T, Error>;
type ContentFilter = fn(&Content) -> bool;

pub struct ContentService<'a> {
    site_repository: Box<dyn SiteRepository + 'a>,
    content_respository: Box<dyn ContentRepository + 'a>,
}

impl<'a> ContentService<'a> {
    pub fn new(environment: &'a Environment) -> ContentService {
        let site_repository = Box::new(LocalSiteRepository::new(environment));
        let content_respository =
            Box::new(LocalContentRepository::new(environment));
        ContentService {
            site_repository,
            content_respository,
        }
    }

    pub fn search<F>(
        &self,
        filter: ContentFilter,
        order: F,
    ) -> Result<Vec<Content>>
    where
        F: FnMut(&Content, &Content) -> Ordering,
    {
        let site_entity = self.site_repository.load()?;
        let content_entities =
            self.content_respository.load_all(&site_entity)?;
        let mut contents = vec![];
        for content_entity in content_entities {
            let content = Content::from(content_entity);
            if filter(&content) {
                contents.push(content);
            }
        }
        contents.sort_by(order);
        return Ok(contents);
    }

    pub fn find(&self, filename: &str) -> Result<Content> {
        let site_entity = self.site_repository.load()?;
        let content = self
            .content_respository
            .load(&site_entity, filename)
            .map(|x| Content::from(x))?;
        return Ok(content);
    }

    pub fn create(&self, target: &str) -> Result<Content> {
        let site_entity = self.site_repository.load()?;
        let content = self
            .content_respository
            .create(&site_entity, target)
            .map(|x| Content::from(x))?;
        return Ok(content);
    }
}
