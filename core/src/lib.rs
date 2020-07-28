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
#[macro_use]
extern crate serde_json;
extern crate itertools;
extern crate simplelog;
#[cfg(test)]
extern crate filesystem;
#[cfg(test)]
extern crate tester;
extern crate uuid;

mod entity;
mod infrastructure;
mod model;
mod repository;
mod service;
mod template;

pub use service::{CommandService, ContentService, SiteService};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
