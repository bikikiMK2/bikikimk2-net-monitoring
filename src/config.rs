use std::fs::File;
use std::io;
use std::io::{BufReader, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub token: String,
    pub prefix: String,
    pub monitoring_channel: u64,
    pub target_url: String,
}

pub fn copy_config_if_not_exists() -> io::Result<()> {
    if !Path::new("config.json").exists() {
        match File::create("config.json") {
            Ok(mut file) => {
                let default_config = include_bytes!("config.json");

                file.write_all(default_config)
            }
            Err(err) => Err(err),
        }
    } else {
        Ok(())
    }
}

pub fn load_config() -> Result<Config> {
    let file = File::open("config.json").unwrap();
    let buf_reader = BufReader::new(file);

    serde_json::from_reader(buf_reader)
}
