extern crate core;
#[macro_use]
extern crate log;
extern crate simple_logger;
extern crate structopt;

use colored::*;
use core::{ContentService, SiteService};
use simple_logger::SimpleLogger;
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
    #[structopt(about = "Create a site")]
    Init,
    #[structopt(about = "Display site information")]
    Info,
    #[structopt(about = "Generate the site files")]
    Generate,
    #[structopt(about = "Publish site")]
    Publish,
    #[structopt(about = "Content command")]
    Content(ContentCommand),
}

#[derive(StructOpt, Debug)]
enum ContentCommand {
    #[structopt(about = "Create a new content")]
    New,
    #[structopt(about = "List contents")]
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
    let result = match matches.command {
        DoorCommand::Init => {
            let site_service = SiteService::new(&environment);
            site_service.create().map(|_| {})
        }
        DoorCommand::Info => {
            let site_service = SiteService::new(&environment);
            site_service.load().map(|site| {
                println!("{}: {}", "Site".bold(), site.title);
                println!("{}:", "Contents".bold());
                todo!()
            })
        }
        DoorCommand::Generate => {
            let site_service = SiteService::new(&environment);
            site_service.generate()
        }
        DoorCommand::Publish => {
            let site_service = SiteService::new(&environment);
            site_service.publish()
        }
        DoorCommand::Content(content_command) => match content_command {
            ContentCommand::New => {
                let content_service = ContentService::new(&environment);
                content_service.create("post").and(Ok(()))
            }
            ContentCommand::List => {
                let content_service = ContentService::new(&environment);
                content_service
                    .search(|_| true, |a, b| a.create_time.cmp(&b.create_time))
                    .map(|contents| {
                        println!("TITLE \t TARGET \t DRAFT \t CREATE TIME");
                        for content in contents {
                            println!(
                                "{} \t {} \t {} \t {}",
                                content.title,
                                content.target,
                                content.draft,
                                content.create_time
                            );
                        }
                    })
            }
        },
    };
    result.map_or_else(
        |e| {
            println!("{}", e);
        },
        |_| {},
    )
}
