use crate::infrastructure::{Environment, Error};
use crate::model::Content;
use crate::repository::{ContentRepository, LocalContentRepository};
use std::cmp::Ordering;

type Result<T> = std::result::Result<T, Error>;
type ContentFilter = fn(&Content) -> bool;

pub struct ContentService<'a> {
    content_respository: Box<dyn ContentRepository + 'a>,
}

impl<'a> ContentService<'a> {
    pub fn new(environment: &'a Environment) -> ContentService {
        let content_respository =
            Box::new(LocalContentRepository::new(environment));
        ContentService {
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
        let content_entities = self.content_respository.load_all()?;
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
        let content = self
            .content_respository
            .load(filename)
            .map(|x| Content::from(x))?;
        return Ok(content);
    }

    pub fn create(&self, target: &str) -> Result<Content> {
        let content = self
            .content_respository
            .create(target)
            .map(|x| Content::from(x))?;
        return Ok(content);
    }
}
