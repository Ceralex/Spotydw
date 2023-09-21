use crate::config::Config;
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::time::{Duration, SystemTime};

const FILE_NAME: &str = "spotify_token.json";

#[derive(Deserialize, Debug)]
struct AuthResponse {
    access_token: String,
    expires_in: usize,
}

#[derive(Serialize, Deserialize)]
pub struct AccessToken {
    token: String,
    expires_at: SystemTime,
}

impl AccessToken {
    pub fn save(&self) -> Result<(), std::io::Error> {
        let path = Config::config_path().unwrap();

        let json_str = serde_json::to_string(&self)?;

        let mut file = File::create(path.join(FILE_NAME))?;
        file.write_all(json_str.as_bytes())?;
        Ok(())
    }

    pub fn load(config: &Config) -> Result<Self, std::io::Error> {
        let path = Config::config_path().unwrap();

        match fs::read_to_string(path.join(FILE_NAME)) {
            Ok(s) => {
                println!("Token file found, checking expiration...");
                let token: AccessToken = serde_json::from_str(&s)?;

                if token.is_expired() {
                    println!("Token expired, getting a new one...");

                    let token = fetch_token(&config).expect("Failed to get access token");

                    token.save().expect("Failed to save access token");
                    return Ok(token);
                }
                println!("Token valid, got it from cache");
                Ok(token)
            }
            Err(_) => {
                println!("Token file not found, requesting a new one...");

                let token = fetch_token(&config).expect("Failed to get access token");

                token.save().expect("Failed to save access token");

                Ok(token)
            }
        }
    }
    pub fn new(token: String, expires_in: usize) -> Self {
        let current_time = SystemTime::now();
        let expires_at = current_time + Duration::from_secs(expires_in as u64);

        AccessToken { token, expires_at }
    }

    pub fn is_expired(&self) -> bool {
        SystemTime::now() >= self.expires_at
    }

    pub fn get_token(&self) -> &str {
        &self.token
    }
}

fn fetch_token(config: &Config) -> Result<AccessToken, ureq::Error> {
    let spotify_id = config.get_spotify_id();
    let spotify_secret = config.get_spotify_secret();

    let auth_header = format!("{}:{}", spotify_id, spotify_secret);
    let encoded_auth_header = general_purpose::STANDARD_NO_PAD.encode(&auth_header);

    let auth_options = [("grant_type", "client_credentials")];

    let response = ureq::post("https://accounts.spotify.com/api/token")
        .set("Authorization", &format!("Basic {encoded_auth_header}"))
        .send_form(&auth_options)
        .unwrap_or_else(|err| {
            eprintln!("ERROR: Failed to make the request: {err}, check your credentials");
            std::process::exit(1);
        });

    let body: AuthResponse = response.into_json().expect("Failed to parse JSON response");

    let access_token = AccessToken::new(body.access_token, body.expires_in);

    Ok(access_token)
}
