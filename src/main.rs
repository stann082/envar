use clap::{App, Arg, SubCommand};
use std::env;

fn main() {
    let matches = App::new("envar")
        .version("1.0")
        .about("Manages Windows environment variables")
        .subcommand(
            SubCommand::with_name("system")
                .about("manages system variables")
                .arg(Arg::with_name("list").short('l').help("lists all system variables")),
        )
        .subcommand(
            SubCommand::with_name("user")
                .about("manages user variables")
                .arg(Arg::with_name("list").short('l').help("lists all user variables")),
        )
        .subcommand(
            SubCommand::with_name("path")
                .about("manages PATH variables")
                .arg(Arg::with_name("list").short('l').help("lists all PATH variables")),
        )
        .get_matches();

    if let Some(_matches) = matches.subcommand_matches("system") {
        if _matches.is_present("list") {
            // Insert code to list system variables
        }
    } else if let Some(_matches) = matches.subcommand_matches("user") {
        if _matches.is_present("list") {
            // Insert code to list user variables
        }
    } else if let Some(_matches) = matches.subcommand_matches("path") {
        if _matches.is_present("list") {
            // Insert code to list path variables
            for (key, value) in env::vars() {
                if key == "PATH" {
                    for path in value.split(';') {
                        println!("{}", path);
                    }
                }
            }
        }
    }
}
