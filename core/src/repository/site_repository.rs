use std::fs::{DirBuilder, File};
use std::io::{Read, Write};
use std::path::Path;

use crate::entity::SiteEntity;
use crate::infrastructure::{Environment, Error};

type Result<T> = std::result::Result<T, Error>;

pub trait SiteRepository {
    fn create(&self) -> Result<SiteEntity>;
    fn load(&self) -> Result<SiteEntity>;
}

pub struct LocalSiteRepository<'a> {
    environment: &'a Environment,
}
impl<'a> LocalSiteRepository<'a> {
    pub fn new(environment: &'a Environment) -> LocalSiteRepository {
        LocalSiteRepository { environment }
    }
}

impl<'a> SiteRepository for LocalSiteRepository<'a> {
    fn create(&self) -> Result<SiteEntity> {
        let path = Path::new(&self.environment.workspace);
        if !path.exists() {
            DirBuilder::new()
                .recursive(true)
                .create(path)
                .map_err(|err| {
                    Error::new("Failed to create the site directory.")
                        .with_inner_error(&err)
                })?;
        }
        let config_path = path.join("site.json");
        if config_path.exists() && config_path.is_file() {
            return Err(Error::new(
                "Failed to create a new site. because a site exists in the current directory.",
            ));
        }
        let mut site = serde_json::from_str::<SiteEntity>("{}").unwrap();
        let mut config_file = File::create(config_path).map_err(|err| {
            Error::new("An error occurred while creating the config file.")
                .with_inner_error(&err)
        })?;
        let content = serde_json::to_string_pretty(&site).map_err(|err| {
            Error::new("An error occurred while converting the config content.")
                .with_inner_error(&err)
        })?;
        config_file.write(&content.into_bytes()).map_err(|err| {
            Error::new("An error occurred while writing the config file.")
                .with_inner_error(&err)
        })?;
        site.root = std::fs::canonicalize(&self.environment.workspace)
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        return Ok(site);
    }
    fn load(&self) -> Result<SiteEntity> {
        trace!("Loading site {0}", self.environment.workspace);
        let path = Path::new(&self.environment.workspace);

        if !path.exists() {
            return Err(Error::new("The dircetory is not exists."));
        }
        let config_path = path.join("site.json");
        if !config_path.exists() {
            return Err(Error::new("The config file is not exists."));
        }

        let mut config_file = File::open(config_path).map_err(|error| {
            Error::new("Failed to open the config file.")
                .with_inner_error(&error)
        })?;
        let mut buffer = String::new();
        config_file.read_to_string(&mut buffer).map_err(|error| {
            Error::new("Failed to read the config file.")
                .with_inner_error(&error)
        })?;

        let mut site =
            serde_json::from_str::<SiteEntity>(&buffer).map_err(|error| {
                Error::new("Failed to resolve the config file.")
                    .with_inner_error(&error)
            })?;
        site.root = std::fs::canonicalize(&self.environment.workspace)
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        return Ok(site);
    }
}

#[cfg(test)]
mod tests {
    use tester::Tester;

    use crate::infrastructure::Environment;

    use super::*;

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
    fn create_site() {
        Tester::new()
            .set_before(clear_site)
            .set_after(clear_site)
            .run(|| {
                let repository = LocalSiteRepository::new(&ENVIRONMENT);
                repository.create().unwrap();
            });
    }

    #[test]
    fn load_site() {
        Tester::new()
            .set_before(clear_site)
            .set_after(clear_site)
            .run(|| {
                let repository = LocalSiteRepository::new(&ENVIRONMENT);
                repository.create().unwrap();
                repository.load().unwrap();
            });
    }
}
