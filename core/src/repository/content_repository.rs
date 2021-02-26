use crate::entity::ContentEntity;
use crate::infrastructure::*;
use crate::template::{DefaultRenderer, Renderer};
use regex::Regex;
use std::fs;
use std::io::Write;
use std::path::Path;

type Result<T> = std::result::Result<T, Error>;

pub trait ContentRepository {
    fn load_all(&self) -> Result<Vec<ContentEntity>>;
    fn create(&self, target: &str) -> Result<ContentEntity>;
    fn load(&self, filename: &str) -> Result<ContentEntity>;
}

pub struct LocalContentRepository<'a> {
    environment: &'a Environment,
}

impl<'a> LocalContentRepository<'a> {
    pub fn new(environment: &'a Environment) -> LocalContentRepository<'a> {
        LocalContentRepository {
            environment: environment,
        }
    }
}

impl<'a> ContentRepository for LocalContentRepository<'a> {
    fn load_all(&self) -> Result<Vec<ContentEntity>> {
        trace!("Loading contents");
        let content_path = &self.environment.content_directory;
        let list = list_files(&content_path, true, |_| true)?;
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
            let content = match self.load(&path) {
                Ok(content) => content,
                Err(err) => {
                    warn!("Failed to load content:{}. error:{}", path, err);
                    continue;
                }
            };
            contents.push(content);
        }
        trace!("Loaded {} content(s)", contents.len());
        return Ok(contents);
    }

    fn create(&self, target: &str) -> Result<ContentEntity> {
        let mut content = serde_json::from_str::<ContentEntity>("{}").unwrap();
        content.path = format!("{}/{}.md", target, content.id);
        content.target = String::from(target);
        let file_path = self
            .environment
            .content_directory
            .join(target)
            .join(format!("{}.md", content.id));

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
        let renderer = DefaultRenderer::new();
        let data = renderer
            .render_string(
                Resource::get_text_content("content.md.hbs"),
                &content,
            )
            .unwrap();
        file.write_all(&mut data.into_bytes()).map_err(|err| {
            Error::new("An error occurred while save file.")
                .with_inner_error(&err)
        })?;

        return Ok(content);
    }

    fn load(&self, filename: &str) -> Result<ContentEntity> {
        trace!("Loading content {}", filename);
        let content_path = &self.environment.content_directory;
        let file_path = content_path.join(filename);
        if !file_path.exists() {
            return Err(Error::new("The file is not exists."));
        }
        let buffer = std::fs::read_to_string(&file_path).map_err(|err| {
            Error::new("Failed to read file.").with_inner_error(&err)
        })?;

        let re = Regex::new(
            r"^\s*``````` json(?P<mark>(.|\s)*?)```````(?P<content>(.|\s)*)",
        )
        .map_err(|err| {
            Error::new("An error occurred while resolving the content.")
                .with_inner_error(&err)
        })?;

        let captures = re
            .captures(&buffer)
            .ok_or(Error::new("Failed to find mark info on the content."))?;

        let mut content = serde_json::from_str::<ContentEntity>(
            &captures["mark"],
        )
        .map_err(|error| {
            Error::new("Failed to convert mark info on the content.")
                .with_inner_error(&error)
        })?;
        content.content = captures["content"].to_string();
        content.more =
            serde_json::from_str(&captures["mark"]).map_err(|error| {
                Error::new("Failed to convert mark info on the content.")
                    .with_inner_error(&error)
            })?;

        content.path = filename.to_string();
        return Ok(content);
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
    fn create_content() {
        let temp_fs = MOCK_FILESYSTEM.temp_dir("mysite").unwrap();
        let workspace = temp_fs.path().to_str().unwrap();
        let environment = Environment::new(".", workspace);
        let content_repository = LocalContentRepository::new(&environment);
        content_repository.create("post").unwrap();
    }

    #[test]
    fn load_content() {
        let temp_fs = MOCK_FILESYSTEM.temp_dir("mysite").unwrap();
        let workspace = temp_fs.path().to_str().unwrap();
        let environment = Environment::new(".", workspace);
        let content_repository = LocalContentRepository::new(&environment);
        content_repository.create("post").unwrap();
        content_repository.load("hello.md").unwrap();
    }

    #[test]
    fn load_all_contents() {
        let temp_fs = MOCK_FILESYSTEM.temp_dir("mysite").unwrap();
        let workspace = temp_fs.path().to_str().unwrap();
        let environment = Environment::new(".", workspace);
        let content_repository = LocalContentRepository::new(&environment);
        content_repository.create("post").unwrap();
        content_repository.load_all().unwrap();
    }
}
