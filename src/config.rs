use std::{fs::File, io::BufReader};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RunConfigJson {
    pub cwd: String,
    pub command: String,
    pub venv: bool,
}

#[derive(Deserialize, Debug)]
pub struct ConfigServerJson {
    pub name: String,
    pub port: i16,
    #[serde(default)]
    pub websocket: bool,
    pub subdomain: Option<String>,
    #[serde(default)]
    pub allow_sub_ext: bool,
    pub run: Option<RunConfigJson>,
}

#[derive(Deserialize, Debug)]
pub struct ConfigJson {
    pub domain: String,
    pub servers: Vec<ConfigServerJson>,
}

pub fn parse_config(filepath: String) -> ConfigJson {
    let file = File::open(filepath).expect("Error opening configuration file.");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).expect("Error parsing configuration JSON.")
}
