use std::fs::{self, DirBuilder, File};
use std::io;
use std::io::{Read, Write};
use std::path::Path;

use colored::*;
use regex::Regex;

use crate::entity::{SiteEntity, ThemeEntity};
use crate::infrastructure::{utilities, Error};

type Result<T> = std::result::Result<T, Error>;

pub trait ThemeRepository {
    fn list(&self) -> Result<Vec<ThemeEntity>>;
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
    fn list(&self) -> Result<Vec<ThemeEntity>> {
        return Ok(vec![]);
    }
    fn load(&self, filename: &str) -> Result<ThemeEntity> {
        return Ok(ThemeEntity {
            name: "".to_string(),
            path: "".to_string(),
        });
    }
}
