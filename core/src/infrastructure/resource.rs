use include_dir::Dir;
use std::path::Path;

static RESOURCE_DIR: Dir = include_dir!("./resource");

fn all_files(root: &Dir) -> Vec<String> {
    let mut result: Vec<String> = root
        .files()
        .into_iter()
        .map(|x| String::from(x.path().to_str().unwrap_or("")))
        .collect();
    for child in root.dirs() {
        result.append(&mut all_files(child))
    }
    return result;
}

pub struct Resource {}

impl Resource {
    pub fn list<S: AsRef<Path>>(path: S) -> Vec<String> {
        let root = RESOURCE_DIR.get_dir(path).unwrap();
        return all_files(&root);
    }

    pub fn get_content<S: AsRef<Path>>(path: S) -> &'static [u8] {
        return RESOURCE_DIR.get_file(path).unwrap().contents();
    }

    pub fn get_text_content<S: AsRef<Path>>(path: S) -> &'static str {
        return RESOURCE_DIR
            .get_file(path)
            .unwrap()
            .contents_utf8()
            .unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_text_content() {
        Resource::get_text_content("site.json.hbs");
    }


    #[test]
    fn list() {
        let list = Resource::list("theme");
        println!("{:?}",list );
    }
}
