extern crate core;
#[macro_use]
extern crate log;
extern crate simple_logger;
extern crate structopt;

use core::{ContentService, Environment, SiteService};
use simple_logger::SimpleLogger;
use std::fs::File;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "door")]
struct ApplicationArguments {
    #[structopt(long, help = "Activate debug mode")]
    debug: bool,
    #[structopt(subcommand)]
    command: DoorCommand,
}
#[derive(StructOpt, Debug)]
enum DoorCommand {
    #[structopt(about = "Init current directory")]
    Init,
    #[structopt(about = "add files to the staging area")]
    Generate {
        #[structopt(short, long, help = "Watch file change")]
        watch: bool,
    },
    #[structopt(about = "add files to the staging area")]
    Publish {
        #[structopt(short, long, help = "Watch file change")]
        watch: bool,
    },
    #[structopt(about = "add files to the staging area")]
    Content(ContentCommand),
}

#[derive(StructOpt, Debug)]
enum ContentCommand {
    New,
    List,
}
fn main() {
    let environment = core::Environment::new(".", ".");
    let matches = ApplicationArguments::from_args();
    if matches.debug {
        SimpleLogger::new()
            .with_level(log::LevelFilter::Trace)
            .init()
            .unwrap();
        debug!("{:?}", matches);
    }
    match matches.command {
        DoorCommand::Init => {
            let site_service = SiteService::new(&environment);
            site_service.create().unwrap();
        }
        DoorCommand::Generate { watch: watch } => {
            let site_service = SiteService::new(&environment);
            site_service.generate().unwrap();
        }
        DoorCommand::Publish { watch: watch } => {
            let site_service = SiteService::new(&environment);
            site_service.publish().unwrap();
        }
        DoorCommand::Content(content_command) => match content_command {
            ContentCommand::New => {
                let content_service = ContentService::new(&environment);
                content_service.create("post").unwrap();
            }
            ContentCommand::List => {
                let content_service = ContentService::new(&environment);
                let contents = content_service
                    .search(
                        |content| true,
                        |a, b| a.create_time.cmp(&b.create_time),
                    )
                    .unwrap();
                for content in contents {
                    println!("{} \t\t {}", content.title, content.create_time);
                }
            }
        },
    }
}
