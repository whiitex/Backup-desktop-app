use std::fs;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub(crate) source: String,
    pub(crate) destination: String,
}

impl Config{

    pub fn new(source: String, destination: String)->Self{
        Self{
            source,
            destination
        }
    }
    pub fn save(&self) {

        if let Err(e) = fs::write("config.json", serde_json::to_string(&self).unwrap()) {
            eprintln!("Failed to save config: {:?}", e);
        }
    }

    pub fn load(&mut self) {
        match fs::read_to_string("config.json") {
            Ok(content) =>{
                if let Ok(config) = serde_json::from_str::<Config>(&content) {
                    self.source = config.source;
                    self.destination = config.destination;
                }
            },
            Err(_e) => {
                self.save();
            }

        }
    }
}
