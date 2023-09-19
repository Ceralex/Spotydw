use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::process;
use serde::{Deserialize, Serialize};

const FOLDER_NAME: &str = "spotydw";
const FILE_NAME: &str = "config.json";

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    spotify_id: String,
    spotify_secret: String,
}
impl Config {
    pub fn save(&self) -> Result<(), std::io::Error> {
        let path = config_path().unwrap();

        let json_str = serde_json::to_string(&self)?;

        let mut file = File::create(path.join(FILE_NAME))?;
        file.write_all(json_str.as_bytes())?;

        Ok(())
    }
    // TODO: Change save format to json
    pub fn load() -> Result<Self, std::io::Error> {
        let path = config_path().unwrap();

        match fs::read_to_string(path.join(FILE_NAME)) {
            Ok(s) => {
               let config: Config = serde_json::from_str(&s)?;

                Ok(config)
            }
            Err(_) => Ok(Config::default()),
        }
    }
    pub fn get_spotify_id(&self) -> &str {
        &self.spotify_id
    }
    pub fn get_spotify_secret(&self) -> &str {
        &self.spotify_secret
    }
    pub fn set_config(&mut self, spotify_id: String, spotify_secret: String) {
        self.spotify_id = spotify_id;
        self.spotify_secret = spotify_secret;
    }
}

pub fn config_path() -> Option<PathBuf> {
    let os = std::env::consts::OS;

    let home_var = match os {
        "linux" | "macos" => env::var("HOME").ok(),
        "windows" => env::var("APPDATA").ok(),
        _ => {
            eprintln!("ERROR: Unsupported OS: {}", os);
            process::exit(1);
        }
    };

    let config_dir = home_var
        .map(|home| {
            let mut path = PathBuf::from(home);

            if os != "windows" {
                path.push(".config");
            }

            path.push(FOLDER_NAME);
            path
        }).unwrap_or_else(|| {
            eprintln!("ERROR: Failed to get home directory");
            process::exit(1);
        });

    fs::create_dir_all(&config_dir).unwrap_or_else(|err| {
        eprintln!("ERROR: Failed to create config directory: {err}");
        process::exit(1);
    });

    Some(config_dir)
}
