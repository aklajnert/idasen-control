mod config;

use crate::config::Config;
use clap::App;
use idasen::Idasen;
use std::process;

pub fn main() -> Result<(), failure::Error> {
    let mut config = Config::new().expect("Failed to load configuration.");
    let mut args = App::new("Desk")
        .version("0.1.0")
        .about("Control the IDASEN desk position via bluetooth.")
        .subcommand(
            App::new("save")
                .about("Save current position under desired name")
                .arg("<NAME> 'Position name'"),
        )
        .subcommand(
            App::new("delete")
                .about("Remove position from configuration")
                .arg("<NAME> 'Position name'"),
        )
        .subcommand(App::new("info").about("Display desk information"));

    for subcommand in config.data.positions.keys() {
        args = args.subcommand(App::new(subcommand).about("Move to saved location"));
    }

    let matches = args.get_matches();

    if let Some(subcommand) = matches.subcommand() {
        match subcommand.0 {
            "save" => {
                let position = subcommand.1.value_of("NAME").unwrap();
                save_position(position, &mut config)
            }
            "delete" => {
                let position = subcommand.1.value_of("NAME").unwrap();
                delete_position(position, &mut config)
            }
            "info" => {
                let idasen = get_desk();
                let current_position = get_desk_position(&idasen);
                println!(
                    "Desk connected\nPosition: {}cm\nAddress: {}",
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

fn get_desk() -> Idasen {
    Idasen::new().expect("Failed to connect to the desk.")
}

fn get_desk_position(idasen: &Idasen) -> u16 {
    idasen.position().expect("Cannot read desk position")
}

fn to_cm(position: u16) -> f32 {
    position as f32 / 100.0
}
