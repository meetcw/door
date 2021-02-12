use std::fs::{self, DirBuilder, File};
use std::io;
use std::io::{Read, Write};
use std::path::Path;

use colored::*;
use regex::Regex;

use crate::entity::{SiteEntity, ThemeEntity};
use crate::infrastructure::*;

type Result<T> = std::result::Result<T, Error>;

pub trait ThemeRepository {
    fn layouts(&self) -> Result<Vec<String>>;
    fn load(&self, filename: &str) -> Result<ThemeEntity>;
}

pub struct LocalThemeRepository<'a> {
    pub site: &'a SiteEntity,
}

impl<'a> LocalThemeRepository<'a> {
    pub fn new(site: &'a SiteEntity) -> LocalThemeRepository<'a> {
        LocalThemeRepository { site }
    }
}

impl<'a> ThemeRepository for LocalThemeRepository<'a> {
    fn layouts(&self) -> Result<Vec<String>> {
        return Ok(vec![]);
    }
    fn load(&self, filename: &str) -> Result<ThemeEntity> {
        return Ok(ThemeEntity {
            name: "".to_string(),
            path: "".to_string(),
        });
    }
}

pub struct DefaultThemeRepository<'a> {
    pub site: &'a SiteEntity,
}

impl<'a> DefaultThemeRepository<'a> {
    pub fn new(site: &'a SiteEntity) -> DefaultThemeRepository<'a> {
        DefaultThemeRepository { site }
    }
}

impl<'a> ThemeRepository for DefaultThemeRepository<'a> {
    fn layouts(&self) -> Result<Vec<String>> {
        return Ok(Resource::list("theme/layouts"));
    }

    fn load(&self, filename: &str) -> Result<ThemeEntity> {
        let config = Resource::get_text_content("theme/theme.json");
        let theme = serde_json::from_str::<ThemeEntity>(config).unwrap();
        return Ok(theme);
    }
}
