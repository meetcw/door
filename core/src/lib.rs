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
extern crate roman;
#[cfg_attr(test, macro_use)]
extern crate serde_json;
extern crate uuid;
extern crate rhai;
extern crate slug;
extern crate path_absolutize;

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
