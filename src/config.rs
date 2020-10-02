use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

const CONFIG_DIR: &str = "~/.config";
const CONFIG_FILE_NAME: &str = "desk.toml";

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ConfigData {
    pub position_down: Option<i16>,
    pub position_up: Option<i16>,
}

#[derive(Debug)]
pub struct Config {
    pub data: ConfigData,
    path: PathBuf,
}

impl Config {
    pub fn new() -> Result<Self, failure::Error> {
        let path = Self::get_path()?;
        let data: ConfigData = match path.exists() {
            true => {
                let mut file = File::open(&path)?;
                let mut file_content = String::new();
                file.read_to_string(&mut file_content)?;
                toml::from_str(&file_content)?
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

    fn get_path() -> Result<PathBuf, std::io::Error> {
        let expanded = shellexpand::tilde(CONFIG_DIR).to_string();
        let config_dir = Path::new(&expanded);

        if !config_dir.exists() {
            let _ = std::fs::create_dir(config_dir)?;
        }

        Ok(config_dir.join(CONFIG_FILE_NAME))
    }
}
