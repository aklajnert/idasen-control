mod config;

use crate::config::Config;
use clap::{App, Arg};
use failure;
use idasen::Idasen;
use std::time::Duration;
use std::{process, thread};

pub fn main() -> Result<(), failure::Error> {
    let matches = App::new("Desk")
        .version("0.1.0")
        .about("Control the IDASEN desk position via bluetooth.")
        .subcommand(App::new("up").about("Move desk up"))
        .subcommand(App::new("down").about("Move desk down"))
        .subcommand(App::new("save-up").about("Save current position as up"))
        .subcommand(App::new("save-down").about("Save current position as down"))
        .subcommand(App::new("info").about("Display desk information"))
        .get_matches();

    if let Some(subcommand) = matches.subcommand() {
        let config = Config::new().expect("Failed to load configuration.");
        let subcommand = subcommand.0;
        if subcommand == "up" && config.data.position_up.is_none() {
            eprintln!(
                "Position `up` is not defined. \
            Please set desk manually to desired position and run `save-up` command."
            );
            process::exit(1);
        } else if subcommand == "down" && config.data.position_down.is_none() {
            eprintln!(
                "Position `down` is not defined. \
            Please set desk manually to desired position and run `save-down` command."
            );
            process::exit(1);
        }
    } else {
        eprintln!("Please select subcommand. Use `help` to see available subcommands.");
        process::exit(1);
    }

    Ok(())
}
