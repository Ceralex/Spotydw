use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::process;

const FOLDER_NAME: &str = "spotydw";
const FILE_NAME: &str = "spotydw.conf";

#[derive(Debug, Default)]
pub struct Config {
    pub spotify_id: String,
    pub spotify_secret: String,
}
impl Config {
    pub fn save(&self) -> Result<(), std::io::Error> {
        let path = config_path().unwrap();

        let mut file = File::create(path.join(FILE_NAME))?;

        writeln!(&mut file, "{}", &self.spotify_id)?;
        writeln!(&mut file, "{}", &self.spotify_secret)?;

        Ok(())
    }
    // TODO: Change save format to json
    pub fn load() -> Result<Self, std::io::Error> {
        let path = config_path().unwrap();

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
