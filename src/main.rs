use idasen::Idasen;
use std::thread;
use std::time::Duration;

pub fn main() -> Result<(), idasen::Error> {
    let desk = Idasen::new()?;

    println!("Desk addr: {}", desk.mac_addr);
    println!("Position: {}", desk.position()?);

    Ok(())
}
