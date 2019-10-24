use std::fs::{self, DirBuilder, File};
use std::io;
use std::io::{Read, Write};
use std::path::Path;

use colored::*;
use regex::Regex;

use crate::entity::{ContentEntity, SiteEntity};
use crate::infrastructure::{utilities, Environment, Error};
use crate::model::Site;
use serde::de::Unexpected::Str;

type Result<T> = std::result::Result<T, Error>;

pub trait ContentRepository {
    fn load_all(&self, site: &SiteEntity) -> Result<Vec<ContentEntity>>;
    fn create(
        &self,
        site: &SiteEntity,
        filename: &str,
        target: &str,
    ) -> Result<ContentEntity>;
    fn load(&self, site: &SiteEntity, filename: &str) -> Result<ContentEntity>;
}

pub struct LocalContentRepository<'a> {
    environment: &'a Environment,
}

impl<'a> LocalContentRepository<'a> {
    pub fn new(environment: &'a Environment) -> LocalContentRepository<'a> {
        LocalContentRepository { environment }
    }
}

impl<'a> ContentRepository for LocalContentRepository<'a> {
    fn load_all(&self, site: &SiteEntity) -> Result<Vec<ContentEntity>> {
        trace!("Loading contents");

        let content_path = Path::new(&site.root).join(&site.content_directory);
        let list = utilities::find_files(&content_path, true, |_| true)?;
        let mut paths = vec![];
        for item in &list {
            let mut path = Path::new(item);
            path = path
                .strip_prefix(content_path.to_str().ok_or(Error::new(
                    &format!("Format of \"path\" is incorrect."),
                ))?)
                .map_err(|err| {
                    Error::new(&format!(
                        "The Path is not The child path of the parent path."
                    ))
                    .with_inner_error(&err)
                })?;
            paths.push(
                path.to_str()
                    .ok_or(Error::new(&format!(
                        "Format of \"path\" is incorrect."
                    )))?
                    .to_string(),
            );
        }
        let mut contents = vec![];
        for path in paths {
            let content = match self.load(site, &path) {
                Ok(content) => content,
                Err(err) => {
                    warn!("Failed to load content:{}. error:{}", path, err);
                    continue;
                }
            };
            if content.target == "POST" {
                contents.push(content);
            }
        }
        trace!("Loaded {} content(s)", contents.len());
        return Ok(contents);
    }

    fn create(
        &self,
        site: &SiteEntity,
        filename: &str,
        target: &str,
    ) -> Result<ContentEntity> {
        println!(
            "{0:>12} {1} {2}",
            "Creating".green().bold(),
            "content",
            filename
        );
        let file_path = Path::new(&site.root)
            .join(&site.content_directory)
            .join(filename);

        if file_path.exists() {
            return Err(Error::new("File already exists."));
        }
        let parent_path = file_path.parent().ok_or(Error::new(
            "An error occurred while getting parent directory from path.",
        ))?;
        if !parent_path.exists() {
            fs::DirBuilder::new()
                .recursive(true)
                .create(parent_path)
                .map_err(|err| {
                    Error::new(
                        "An error occurred while creating parent directory.",
                    )
                    .with_inner_error(&err)
                })?;
        }

        let mut file = fs::File::create(&file_path).map_err(|err| {
            Error::new("An error occurred while creating file.")
                .with_inner_error(&err)
        })?;
        let mut content = serde_json::from_str::<ContentEntity>("{}").unwrap();
        content.path = String::from(filename);
        content.target = String::from(target);
        let mut value = serde_json::to_value(content.clone()).unwrap();
        let map = value.as_object_mut().unwrap();
        map.remove("content").unwrap();
        let mark = serde_json::to_string_pretty(&map)
            .map_err(|err| {
                Error::new("An error occurred while save file.")
                    .with_inner_error(&err)
            })?
            .to_string();
        let data = format!(
            "``````` json\r\n{}\r\n```````\r\n{}",
            mark, content.content
        );
        file.write_all(&mut data.into_bytes()).map_err(|err| {
            Error::new("An error occurred while save file.")
                .with_inner_error(&err)
        })?;

        return Ok(content);
    }

    fn load(&self, site: &SiteEntity, filename: &str) -> Result<ContentEntity> {
        trace!("Loading content {}", filename);
        let content_path = Path::new(&site.root).join(&site.content_directory);
        let file_path = content_path.join(filename);
        if !file_path.exists() {
            return Err(Error::new("The file is not exists."));
        }
        let mut file = fs::File::open(file_path).map_err(|err| {
            Error::new("Failed to open the file.").with_inner_error(&err)
        })?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).map_err(|err| {
            Error::new("Failed to read file.").with_inner_error(&err)
        })?;

        let re = Regex::new(
            r"^\s*``````` json(?P<mark>(.|\s)*?)```````(?P<content>(.|\s)*)",
        )
        .map_err(|err| {
            Error::new("An error occurred while resolving the content.")
                .with_inner_error(&err)
        })?;

        let caps = re
            .captures(&buffer)
            .ok_or(Error::new("Failed to find mark info on the content."))?;

        let mut content = serde_json::from_str::<ContentEntity>(&caps["mark"])
            .map_err(|error| {
                Error::new("Failed to convert mark info on the content.")
                    .with_inner_error(&error)
            })?;

        content.content = caps["content"].to_string();
        content.path = filename.to_string();
        return Ok(content);
    }
}
#[cfg(test)]
mod tests {
    use tester::Tester;

    use crate::infrastructure::Environment;

    use super::*;
    use crate::repository::{LocalSiteRepository, SiteRepository};

    lazy_static! {
        static ref ENVIRONMENT: Environment =
            Environment::new(".", "/tmp/mysite");
    }
    fn clear_site() {
        let path = std::path::Path::new(&ENVIRONMENT.workspace);
        if path.exists() {
            std::fs::remove_dir_all(path).unwrap();
        };
    }

    #[test]
    fn create_content() {
        Tester::new()
            .set_before(clear_site)
            .set_after(clear_site)
            .run(|| {
                let site_repository = LocalSiteRepository::new(&ENVIRONMENT);
                let site = site_repository.create().unwrap();
                let content_repository =
                    LocalContentRepository::new(&ENVIRONMENT);
                content_repository
                    .create(&site, "hello.md", "post")
                    .unwrap();
            })
    }

    #[test]
    fn load_content() {
        Tester::new()
            .set_before(clear_site)
            .set_after(clear_site)
            .run(|| {
                let site_repository = LocalSiteRepository::new(&ENVIRONMENT);
                let site = site_repository.create().unwrap();
                let content_repository =
                    LocalContentRepository::new(&ENVIRONMENT);
                content_repository
                    .create(&site, "hello.md", "post")
                    .unwrap();
                content_repository.load(&site, "hello.md").unwrap();
            })
    }

    #[test]
    fn load_all_contents() {
        Tester::new()
            .set_before(clear_site)
            .set_after(clear_site)
            .run(|| {
                let site_repository = LocalSiteRepository::new(&ENVIRONMENT);
                let site = site_repository.create().unwrap();
                let content_repository =
                    LocalContentRepository::new(&ENVIRONMENT);
                content_repository
                    .create(&site, "hello.md", "post")
                    .unwrap();
                content_repository.load_all(&site).unwrap();
            })
    }
}
