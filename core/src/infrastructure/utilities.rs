use super::Error;
use std;
use std::fs::*;
use std::path::Path;
use std::rc::Rc;

type Result<T> = std::result::Result<T, Error>;
type Filter = fn(&Path) -> bool;

fn innser_find_files(root: &Path, recursive: bool) -> Result<Vec<String>> {
    let mut list: Vec<String> = vec![];
    if !root.exists() {
        return Ok(vec![]);
    } else if root.is_dir() {
        for entry in read_dir(root).map_err(|err| {
            Error::new("Failed to open Directory.").with_inner_error(&err)
            // TODO:: log permission denied
        })? {
            let entry = entry.map_err(|err| {
                Error::new("Failed to open Directory.").with_inner_error(&err)
            })?;
            let sub_path = entry.path();
            if sub_path.is_dir() && recursive {
                let mut sub_list = innser_find_files(&sub_path, recursive)?;
                list.append(&mut sub_list);
            } else if sub_path.is_file() {
                let tmp = sub_path.as_path();
                list.push(tmp.to_str().unwrap().to_string());
            }
        }
    } else {
        list.push(root.to_str().unwrap().to_string());
    }
    return Ok(list);
}

pub fn find_files(
    root: &Path,
    recursive: bool,
    filter: Filter,
) -> Result<Vec<String>> {
    let mut list = vec![];
    let absolute_list = innser_find_files(root, recursive).unwrap();
    for item in absolute_list {
        let path = Path::new(&item);
        if filter(path) {
            list.push(path.to_str().unwrap().to_string());
        }
    }
    return Ok(list);
}

pub fn copy_files(source: &Path, target: &Path, filter: Filter) -> Result<()> {
    if source.is_file() {
        return Err(Error::new("Source path must be a directory."));
    }
    if target.is_file() {
        return Err(Error::new("Target path must be a directory."));
    }
    if !target.exists() {
        DirBuilder::new()
            .recursive(true)
            .create(target)
            .map_err(|error| {
                Error::new(
                    "An error occurred while creating the target directory.",
                )
                .with_inner_error(&error)
            })?;
    }
    let files = find_files(&source, true, filter)?;
    for file in &files {
        let source_file_path = Path::new(file);
        let path = source_file_path
            .strip_prefix(source.to_str().ok_or(Error::new(&format!(
                "Format of \"source\" is incorrect."
            )))?)
            .map_err(|err| {
                Error::new(&format!(
                    "The Path is not The child path of the parent path."
                ))
                .with_inner_error(&err)
            })?;
        let target_file_path = target.join(path);

        let parent_path = target_file_path
            .parent()
            .ok_or(Error::new("Failed to get parent directory."))?;
        DirBuilder::new()
            .recursive(true)
            .create(parent_path)
            .map_err(|error| {
                Error::new(
                    "An error occurred while creating the parent directory.",
                )
                .with_inner_error(&error)
            })?;
        copy(&source_file_path, &target_file_path).map_err(|error| {
            Error::new("Failed to copy file.").with_inner_error(&error)
        })?;
    }
    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::*;
    use tester::Tester;

    #[test]
    fn find_files() {
        Tester::new().run(|| {
            let list =
                super::find_files(&Path::new("."), true, |_| true).unwrap();
            assert!(list.len() > 0);
        })
    }
}
