use std::collections::HashMap;
use std::{process, thread, time};

use clap::{crate_description, crate_name, crate_version, App, Arg, ArgMatches};
use idasen::{get_instance, Device, Idasen};

use crate::config::Config;
use chrono::{DateTime, Local};
use std::time::SystemTime;

mod config;

#[tokio::main]
async fn main() -> Result<(), failure::Error> {
    let mut config = Config::new().expect("Failed to load configuration.");
    let mut args = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
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
        .subcommand(App::new("info").about("Display desk information"))
        .subcommand(App::new("config").about("Show config file content"));

    let subcommands = config
        .data
        .positions
        .iter()
        .map(|(name, value)| (name.clone(), format!("Move to {}cm", to_cm(*value))))
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
                save_position(&position, &mut config).await
            }
            "delete" => {
                let position = get_name_from_args(subcommand.1);
                delete_position(&position, &mut config)
            }
            "info" => {
                let idasen = get_desk(&config).await;
                let current_position = get_desk_position(&idasen);
                println!(
                    "Position: {}cm\nAddress: {}",
                    to_cm(current_position.await),
                    idasen.mac_addr
                );
            }
            "config" => {
                println!(
                    "Configuration loaded from: {:?}\n\n{}",
                    config.path, config.data
                );
            }
            value => move_to(value, &mut config).await,
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

async fn move_to(position: &str, config: &mut Config) {
    let desired_position = *config.data.positions.get(position).unwrap();
    let idasen = get_desk(config).await;
    let current_position = get_desk_position(&idasen).await;
    println!(
        "Moving desk to position: {} ({}cm -> {}cm)...",
        position,
        to_cm(current_position),
        to_cm(desired_position)
    );
    idasen
        .move_to_with_progress(desired_position)
        .await
        .expect("Failed to move the desk");
    let current_position = get_desk_position(&idasen).await;
    if current_position != desired_position {
        println!("Slightly adjusting position...");
        idasen
            .move_to(desired_position)
            .await
            .expect("Failed to adjust desk position.");
    }
    let system_time = SystemTime::now();
    let datetime: DateTime<Local> = system_time.into();

    println!(
        "Desk moved at {}. Desired position: {}cm, achieved position: {}cm",
        datetime.format("%H:%M"),
        to_cm(desired_position),
        to_cm(current_position)
    );
}

async fn save_position(position: &str, config: &mut Config) {
    let position = match position {
        "info" | "save" | "delete" => {
            eprintln!("Cannot overwrite a reserved keyword: {}", position);
            process::exit(1);
        }
        _ => position,
    };
    let idasen = get_desk(config).await;
    let current_position = get_desk_position(&idasen).await;
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

async fn get_desk(config: &Config) -> Idasen<impl Device> {
    println!("Connecting to the desk...");
    let max_attempts = config.get_connection_attempts();
    let mut attempt = 1;
    loop {
        match get_instance().await {
            Ok(desk) => {
                println!("Connected successfully.");
                return desk;
            }
            Err(error) => {
                if attempt >= max_attempts {
                    eprintln!("Failed to connect to the desk: {}.", error);
                    process::exit(1);
                } else {
                    attempt += 1;
                    let sleep_duration = time::Duration::from_millis(100 * attempt);
                    thread::sleep(sleep_duration);
                }
            }
        }
    }
}

async fn get_desk_position(idasen: &Idasen<impl Device>) -> u16 {
    idasen.position().await.expect("Cannot read desk position")
}

fn to_cm(position: u16) -> f32 {
    position as f32 / 100.0
}
