use crate::infrastructure::Environment;
use clap::{App, Arg, SubCommand};
use colored::*;

pub fn run() {
    let app = App::new("Door")
        .version("1.0")
        .author("Walter . <defaultwalter@gmail.com>")
        .about("A Simple Static Site Generator.")
        .version_short("v")
        .subcommand(
            SubCommand::with_name("init")
                .about("Create a new site in the current directory.")
                .display_order(1),
        )
        .subcommand(
            SubCommand::with_name("info")
                .about("Show info of the current site.")
                .display_order(2),
        )
        .subcommand(
            SubCommand::with_name("build")
                .about("Build the current site.")
                .display_order(3),
        )
        .subcommand(
            SubCommand::with_name("publish")
                .about("Publish the current site.")
                .display_order(4),
        )
        .subcommand(
            SubCommand::with_name("server")
                .about("Lanuch server.")
                .arg(
                    Arg::with_name("PORT")
                        .help("Port of the server.")
                        .default_value("8765")
                        .takes_value(true),
                )
                .display_order(5),
        )
        .subcommand(
            SubCommand::with_name("content")
                .about("Some commands about content.")
                .subcommand(
                    SubCommand::with_name("new")
                        .about("Create a new content.")
                        .arg(
                            Arg::with_name("PATH")
                                .help("File name of the new content.")
                                .required(true),
                        )
                        .arg(
                            Arg::with_name("draft")
                                .help("Create a new draft content.")
                                .short("d")
                                .long("draft"),
                        )
                        .display_order(0),
                )
                .subcommand(
                    SubCommand::with_name("list")
                        .about("List all contents.")
                        .display_order(1),
                )
                .display_order(6),
        );
    let matches = app.get_matches();
    handle_matches(matches);
}

fn handle_matches(matches: clap::ArgMatches) {
    let environment = Environment::new(".", ".");
    if let Some(_) = matches.subcommand_matches("init") {
        // if let Err(error) = site_command::init() {
        //     error!("{}", error);
        // } else {
        //     println!("{0:>12}", "Finished".green().bold());
        // }
        return;
    };
    if let Some(_) = matches.subcommand_matches("info") {
        // if let Err(error) = site_command::info(".") {
        //     error!("{}", error);
        // } else {
        //     println!("{0:>12}", "Finished".green().bold());
        // }
        return;
    };
    if let Some(_) = matches.subcommand_matches("build") {
        // if let Err(error) = site_command::build(".") {
        //     error!("{}", error);
        // } else {
        //     println!("{0:>12}", "Finished".green().bold());
        // }
        return;
    };
    if let Some(_) = matches.subcommand_matches("publish") {
        // if let Err(error) = site_command::publish(".") {
        //     error!("{}", error);
        // } else {
        //     println!("{0:>12}", "Finished".green().bold());
        // }
        return;
    };
    if let Some(matches) = matches.subcommand_matches("server") {
        // let port = matches
        //     .value_of("PORT")
        //     .unwrap_or("8765")
        //     .parse::<u64>()
        //     .unwrap_or(8765);
        // if let Err(error) = site_command::server(".", port) {
        //     error!("{}", error);
        // } else {
        //     println!("{0:>12}", "Finished".green().bold());
        // }
        return;
    };
    if let Some(matches) = matches.subcommand_matches("content") {
        if let Some(matches) = matches.subcommand_matches("new") {
            // let path = matches.value_of("PATH").unwrap_or(".");
            // if let Err(error) = content_command::new(".", path) {
            //     error!("{}", error);
            // } else {
            //     println!("{0:>12}", "Finished".green().bold());
            // }
            return;
        }
        if let Some(_) = matches.subcommand_matches("list") {
            // if let Err(error) = content_command::list(".") {
            //     error!("{}", error);
            // } else {
            //     println!("{0:>12}", "Finished".green().bold());
            // }
            return;
        }
        println!("{}", matches.usage());
        return;
    };
    println!("{}", matches.usage());
}
