use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    min_height: Option<i16>,
    max_height: Option<i16>,
}

impl Config {
    pub fn load() -> Self {
        toml::from_str(
            r#"
        ip = '127.0.0.1'

        [keys]
        github = 'xxxxxxxxxxxxxxxxx'
        travis = 'yyyyyyyyyyyyyyyyy'
    "#,
        )
        .unwrap()
    }

    pub fn save(&mut self) {
        self.max_height = Some(7400);
        self.min_height = Some(12200);
        println!("TOML: \n{}", toml::to_string(&self).unwrap());
    }
}
