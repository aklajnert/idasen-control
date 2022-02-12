use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

const CONFIG_DIR: &str = "~/.config";
const CONFIG_FILE_NAME: &str = "desk.toml";
const DEFAULT_CONNECTION_ATTEMPTS: u64 = 5;

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigData {
    pub positions: HashMap<String, u16>,
    pub connection_attempts: Option<u64>,
}

impl Default for ConfigData {
    fn default() -> Self {
        ConfigData {
            positions: HashMap::new(),
            connection_attempts: Some(DEFAULT_CONNECTION_ATTEMPTS),
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub data: ConfigData,
    pub path: PathBuf,
}

impl Config {
    pub fn new() -> Result<Self, failure::Error> {
        let path = Self::get_path()?;
        let data: ConfigData = match path.exists() {
            true => {
                let mut file = File::open(&path)?;
                let mut file_content = String::new();
                file.read_to_string(&mut file_content)?;
                match toml::from_str(&file_content) {
                    Ok(config) => config,
                    // for errors in TOML structure just reset it to defaults
                    Err(_) => ConfigData::default(),
                }
            }
            false => ConfigData::default(),
        };
        Ok(Self { data, path })
    }

    pub fn save(&mut self) -> Result<(), failure::Error> {
        let mut file = match self.path.exists() {
            true => File::create(&self.path)?,
            false => File::create(&self.path)?,
        };

        let new_content = toml::to_string(&self.data)?;
        let _ = file.write_all(new_content.as_bytes())?;
        Ok(())
    }

    pub fn get_connection_attempts(&self) -> u64 {
        self.data
            .connection_attempts
            .unwrap_or(DEFAULT_CONNECTION_ATTEMPTS)
    }

    fn get_path() -> Result<PathBuf, std::io::Error> {
        let expanded = shellexpand::tilde(CONFIG_DIR).to_string();
        let config_dir = Path::new(&expanded);

        if !config_dir.exists() {
            let _ = std::fs::create_dir(config_dir)?;
        }

        Ok(config_dir.join(CONFIG_FILE_NAME))
    }
}

impl Display for ConfigData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "connection_attempts: {}\npositions:\n{}",
            self.connection_attempts
                .unwrap_or(DEFAULT_CONNECTION_ATTEMPTS),
            self.positions
                .iter()
                .map(|(key, value)| format!("  {}: {}", key, value))
                .join("\n")
        )
    }
}
