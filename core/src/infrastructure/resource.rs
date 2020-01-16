use include_dir::Dir;

pub static RESOURCE: Dir = include_dir!("./resource");

#[cfg(test)]
mod tests {
    use super::*;
    use include_dir::*;

    #[test]
    fn resource_is_works() {
        RESOURCE.get_file("site_template/site.json.hbs").unwrap();
        let files = RESOURCE.files();
        //        assert!(files.len() > 0);
    }
}
