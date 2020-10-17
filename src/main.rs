mod config;

use crate::config::Config;
use clap::{App, Arg, ArgMatches};
use idasen::{get_instance, Device, Idasen};
use std::collections::HashMap;
use std::process;

pub fn main() -> Result<(), failure::Error> {
    let mut config = Config::new().expect("Failed to load configuration.");
    let mut args = App::new("Desk")
        .version("0.1.4")
        .about("Control the IDASEN desk position via bluetooth.")
        .subcommand(
            App::new("save")
                .about("Save current position under desired name")
                .arg(Arg::with_name("name").help("Position name")),
        )
        .subcommand(
            App::new("delete")
                .about("Remove position from configuration")
                .arg(Arg::with_name("name").help("Position name")),
        )
        .subcommand(App::new("info").about("Display desk information"));

    let subcommands = config
        .data
        .positions
        .iter()
        .map(|(name, value)| {
            (
                name.clone(),
                format!("Move to {}cm", to_cm(*value)).to_string(),
            )
        })
        .collect::<HashMap<String, String>>();

    for (name, about) in subcommands.iter() {
        args = args.subcommand(App::new(name).about(about.as_str()));
    }

    let matches = args.get_matches();

    let subcommand = matches.subcommand();
    if !subcommand.0.is_empty() {
        match subcommand.0 {
            "save" => {
                let position = get_name_from_args(subcommand.1);
                save_position(&position, &mut config)
            }
            "delete" => {
                let position = get_name_from_args(subcommand.1);
                delete_position(&position, &mut config)
            }
            "info" => {
                let idasen = get_desk();
                let current_position = get_desk_position(&idasen);
                println!(
                    "Position: {}cm\nAddress: {}",
                    to_cm(current_position),
                    idasen.mac_addr
                );
            }
            value => move_to(value, &mut config),
        };
    } else {
        eprintln!("Please select subcommand. Use `help` to see available subcommands.");
        process::exit(1);
    }

    Ok(())
}

fn get_name_from_args(args: Option<&ArgMatches>) -> String {
    match args.unwrap().value_of("name") {
        Some(value) => value.to_string(),
        None => {
            eprintln!("Missing position name.");
            process::exit(1);
        }
    }
}

fn move_to(position: &str, config: &mut Config) {
    let desired_position = *config.data.positions.get(position).unwrap();
    let idasen = get_desk();
    let current_position = get_desk_position(&idasen);
    println!(
        "Moving desk to position: {} ({}cm -> {}cm)...",
        position,
        to_cm(current_position),
        to_cm(desired_position)
    );
    idasen
        .move_to_with_progress(desired_position)
        .expect("Failed to move the desk");
    let current_position = get_desk_position(&idasen);
    if current_position != desired_position {
        println!("Slightly adjusting position...");
        idasen
            .move_to(desired_position)
            .expect("Failed to adjust desk position.");
    }
    println!(
        "Desk moved. Desired position: {}cm, achieved position: {}cm",
        to_cm(desired_position),
        to_cm(current_position)
    );
}

fn save_position(position: &str, config: &mut Config) {
    let position = match position {
        "info" | "save" | "delete" => {
            eprintln!("Cannot overwrite a reserved keyword: {}", position);
            process::exit(1);
        }
        _ => position,
    };
    let idasen = get_desk();
    let current_position = get_desk_position(&idasen);
    let entry = config
        .data
        .positions
        .entry(position.to_string())
        .or_default();
    *entry = current_position;

    config.save().expect("Failed to save configuration");
    println!(
        "Position `{}` ({}cm) saved in configuration file",
        position,
        to_cm(current_position)
    );
}

fn delete_position(position: &str, config: &mut Config) {
    match config.data.positions.remove(position) {
        Some(_) => {
            config.save().expect("Failed to save configuration");
            println!("Position `{}` removed from configuration file", position);
        }
        None => {
            println!(
                "Position `{}` doesn't exist in configuration file",
                position
            );
        }
    }
}

fn get_desk() -> Idasen<impl Device> {
    println!("Connecting to the desk...");
    let mut attempt = 0;
    loop {
        match get_instance() {
            Ok(desk) => {
                println!("Connected successfully on attempt {}.", attempt);
                return desk;
            }
            Err(_) => {
                if attempt > 3 {
                    eprintln!("Failed to connect to the desk.");
                    process::exit(1);
                } else {
                    attempt += 1;
                }
            }
        }
    }
}

fn get_desk_position(idasen: &Idasen<impl Device>) -> u16 {
    idasen.position().expect("Cannot read desk position")
}

fn to_cm(position: u16) -> f32 {
    position as f32 / 100.0
}
