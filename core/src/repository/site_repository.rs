use std::fs::{DirBuilder, File};
use std::io::{Read, Write};
use std::path::Path;

use crate::entity::SiteEntity;
use crate::infrastructure::{Environment, Error, RESOURCE};
use crate::template::{DefaultRenderer, Renderer};

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
        let site_config_path = path.join("site.json");
        if site_config_path.exists() && site_config_path.is_file() {
            return Err(Error::new(
                "Failed to create a new site. because a site exists in the current directory.",
            ));
        }
        let mut site = serde_json::from_str::<SiteEntity>("{}").unwrap();

        let mut site_config_file =
            File::create(site_config_path).map_err(|err| {
                Error::new("An error occurred while creating the config file.")
                    .with_inner_error(&err)
            })?;
        let renderer = DefaultRenderer::new();
        let content = renderer
            .render_template(
                RESOURCE
                    .get_file("site_template/site.json.hbs")
                    .unwrap()
                    .contents_utf8()
                    .unwrap(),
                &site,
            )
            .unwrap();
        site_config_file
            .write(&content.into_bytes())
            .map_err(|err| {
                Error::new("An error occurred while writing the config file.")
                    .with_inner_error(&err)
            })?;

        println!("{:?}", self.environment.workspace);
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
    use filesystem::{FakeFileSystem, TempDir, TempFileSystem};

    use crate::infrastructure::Environment;

    use super::*;

    lazy_static! {
        static ref MOCK_FILESYSTEM: FakeFileSystem = FakeFileSystem::new();
    }

    #[test]
    fn create_site() {
        let temp_fs = MOCK_FILESYSTEM.temp_dir("mysite").unwrap();
        let workspace = temp_fs.path().to_str().unwrap();
        let environment = Environment::new(".", workspace);
        let repository = LocalSiteRepository::new(&environment);
        repository.create().unwrap();
    }

    #[test]
    fn load_site() {
        let temp_fs = MOCK_FILESYSTEM.temp_dir("mysite").unwrap();
        let workspace = temp_fs.path().to_str().unwrap();
        let environment = Environment::new(".", workspace);
        let repository = LocalSiteRepository::new(&environment);
        repository.create().unwrap();
        repository.load().unwrap();
    }
}
