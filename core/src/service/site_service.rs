use crate::infrastructure::{Environment, Error};
use crate::model::{Content, Site};

type Result<T> = std::result::Result<T, Error>;
type ContentFilter = fn(&Content) -> bool;
use crate::repository::{
    ContentRepository, LocalContentRepository, LocalSiteRepository,
    SiteRepository,
};
use std::cmp::Ordering;

pub struct SiteService<'a> {
    site_repository: Box<dyn SiteRepository + 'a>,
}

impl<'a> SiteService<'a> {
    pub fn new(environment: &'a Environment) -> SiteService {
        let site_repository = Box::new(LocalSiteRepository::new(environment));

        SiteService { site_repository }
    }

    pub fn create(&self) -> Result<Site> {
        self.site_repository.create().map(|x| Site::from(x))
    }

    pub fn load(&self) -> Result<Site> {
        self.site_repository.load().map(|x| Site::from(x))
    }
}
