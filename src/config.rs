use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::{fs, process};

const FOLDER_NAME: &str = "spotydw";
const FILE_NAME: &str = "spotydw.conf";

#[derive(Debug, Default)]
pub struct Config {
    spotify_id: String,
    spotify_secret: String,
}

impl Config {
    pub fn save(&self) -> Result<(), std::io::Error> {
        let path = config_path().unwrap_or(std::env::current_dir().unwrap());

        let mut file = File::create(path.join(FILE_NAME))?;

        writeln!(&mut file, "{}", &self.spotify_id)?;
        writeln!(&mut file, "{}", &self.spotify_secret)?;

        Ok(())
    }

    pub fn set(&mut self, id: String, secret: String) {
        self.spotify_id = id;
        self.spotify_secret = secret;
    }

    pub fn load() -> Result<Self, std::io::Error> {
        let path = config_path().unwrap_or(std::env::current_dir().unwrap());

        dbg!(&path.join(FILE_NAME));
        match fs::read_to_string(path.join(FILE_NAME)) {
            Ok(s) => {
                let mut lines = s.lines();

                let id = lines.next().unwrap_or_default().to_string();
                let secret = lines.next().unwrap_or_default().to_string();

                Ok(Config {
                    spotify_id: id,
                    spotify_secret: secret,
                })
            }
            Err(_) => Ok(Config::default()),
        }
    }
}

fn config_path() -> Result<PathBuf, std::env::VarError> {
    let os = std::env::consts::OS;

    let config_dir = match os {
        "linux" | "macos" => PathBuf::from(std::env::var("HOME")?).join(".config"),
        "windows" => PathBuf::from(std::env::var("APPDATA")?),
        _ => {
            eprintln!("ERROR: unsupported OS: {}", os);
            process::exit(1)
        }
    };

    let config_path = config_dir.join(FOLDER_NAME);

    std::fs::create_dir_all(&config_path).unwrap();

    Ok(config_path)
}
