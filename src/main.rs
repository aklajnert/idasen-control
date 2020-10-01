mod config;

use crate::config::Config;
use failure;
use idasen::Idasen;
use std::thread;
use std::time::Duration;

pub fn main() -> Result<(), failure::Error> {
    let mut config = Config::new()?;
    config.save();
    println!("{:?}", config);
    let desk = Idasen::new()?;

    println!("Desk addr: {}", desk.mac_addr);
    println!("Position: {}", desk.position()?);

    Ok(())
}
