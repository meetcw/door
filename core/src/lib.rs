extern crate chrono;
#[cfg(test)]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate regex;
#[macro_use]
extern crate include_dir;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[cfg(test)]
extern crate filesystem;
extern crate itertools;
extern crate pulldown_cmark;
#[cfg_attr(test, macro_use)]
extern crate serde_json;
#[cfg(test)]
extern crate tester;
extern crate uuid;
extern crate roman;

#[macro_export]
macro_rules! function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        &name[..name.len() - 3]
    }};
}
mod entity;
mod infrastructure;
mod model;
mod repository;
mod service;
mod template;

pub use infrastructure::Environment;
pub use service::{ContentService, SiteService};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
