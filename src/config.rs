use std::{env, fs};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub source: String,
    pub destination: String,
    pub extension: String,
}

impl Config{

    pub fn new(source: String, destination: String, extension: String)->Self{
        Self{
            source,
            destination,
            extension,
        }
    }
    pub fn save(&self) {
        let config_path = Config::get_path();

        if let Err(e) = fs::write(&config_path, serde_json::to_string(&self).unwrap()) {
            eprintln!("Failed to save config: {:?}", e);
        }
    }

    pub fn load(&mut self) {
        let config_path = Config::get_path();

        match fs::read_to_string(&config_path) {
            Ok(content) =>{
                if let Ok(config) = serde_json::from_str::<Config>(&content) {
                    self.source = config.source;
                    self.destination = config.destination;
                    self.extension = config.extension;
                }
            },
            Err(_e) => {
                self.save();
            }

        }
    }

    pub fn get_path()->PathBuf{
        let exe_path = env::current_exe().expect("Failed to get current executable path");
        let assets_path = exe_path.parent().unwrap().join("assets");
        assets_path.join("config.json")
    }
}
