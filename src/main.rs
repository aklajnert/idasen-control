mod config;

use crate::config::Config;
use clap::App;
use failure;
use idasen::Idasen;
use std::process;

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
        let mut config = Config::new().expect("Failed to load configuration.");
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

        let idasen = Idasen::new().expect("Failed to connect to the desk.");

        match subcommand {
            "down" => move_to("down", &mut config, &idasen),
            "up" => move_to("up", &mut config, &idasen),
            "save-down" => save_position("down", &mut config, &idasen),
            "save-up" => save_position("up", &mut config, &idasen),
            "info" => {
                let current_position = get_desk_position(&idasen);
                println!(
                    "Desk connected\nPosition: {}\nAddress: {}",
                    current_position, idasen.mac_addr
                );
            }
            _ => (),
        };
    } else {
        eprintln!("Please select subcommand. Use `help` to see available subcommands.");
        process::exit(1);
    }

    Ok(())
}

fn move_to(position: &str, config: &mut Config, idasen: &Idasen) {
    let desired_position = match position {
        "up" => config.data.position_up.unwrap(),
        "down" => config.data.position_down.unwrap(),
        _ => 0,
    };
    println!("Moving desk {}...", position);
    idasen
        .move_to(desired_position)
        .expect("Failed to move the desk");
    let current_position = get_desk_position(&idasen);
    if current_position != desired_position {
        println!("Slightly adjusting position...");
        idasen
            .move_to(desired_position)
            .expect("Failed to adjust desk position.");
    }
    println!(
        "Desk moved. Desired position: {}, achieved position: {}",
        desired_position, current_position
    );
}

fn save_position(position: &str, config: &mut Config, idasen: &Idasen) {
    let current_position = get_desk_position(&idasen);
    match position {
        "down" => config.data.position_down = Some(current_position),
        "up" => config.data.position_up = Some(current_position),
        _ => (),
    };

    config.save().expect("Failed to save configuration");
    println!("Position `{}` saved in configuration file", position);
}

fn get_desk_position(idasen: &Idasen) -> i16 {
    idasen.position().expect("Cannot read desk position")
}
