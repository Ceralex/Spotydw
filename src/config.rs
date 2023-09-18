use std::fs;
use std::fs::File;
use std::io::Write;

const FILE_NAME: &str = "spotydw.conf";

#[derive(Debug, Default)]
pub struct Config {
    spotify_id: String,
    spotify_secret: String,
}

impl Config {
    // TODO: change file location
    pub fn save(&self) -> Result<(), std::io::Error> {
        let mut file = File::create(FILE_NAME)?;

        writeln!(&mut file, "{}", &self.spotify_id)?;
        writeln!(&mut file, "{}", &self.spotify_secret)?;

        Ok(())
    }

    pub fn set(&mut self, id: String, secret: String) {
        self.spotify_id = id;
        self.spotify_secret = secret;
    }

    pub fn load() -> Result<Self, std::io::Error> {
        match fs::read_to_string(FILE_NAME) {
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
