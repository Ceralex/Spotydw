use std::env;
use std::process::{Command, ExitCode};
use which::which;

mod config;
mod ffmpeg;
mod parser;
mod spotify {
    pub mod access_token;
    pub mod api;
    pub mod download;
}
mod youtube {
    pub mod api;
    pub mod download;
}
use config::Config;
use parser::parse_url;
use parser::UrlType::{SoundCloud, Spotify};
use parser::{SoundCloudType, SpotifyType};
use spotify::access_token::AccessToken;
use spotify::download::{download_album, download_playlist, download_track};

fn usage(program: &str) {
    eprintln!("Usage: {program} [SUBCOMMAND] [OPTIONS]");
    eprintln!("Subcommands:");
    eprintln!("     config <SPOTIFY_CLIENT_ID> <SPOTIFY_CLIENT_SECRET>");
    eprintln!("     download <URL>");
}

fn entry() -> Result<(), ()> {
    let mut args = env::args();
    let program = args.next().expect("Path to program is provided");

    let subcommand = args.next().ok_or_else(|| {
        usage(&program);
        eprintln!("ERROR: no subcommand is provided");
    })?;

    let mut config = Config::load().map_err(|err| {
        eprintln!("ERROR: failed to load config: {err})");
    })?;

    match subcommand.as_str() {
        "config" => {
            let id = args.next().ok_or_else(|| {
                usage(&program);
                eprintln!("ERROR: Spotify client is not provided");
            })?;
            let secret = args.next().ok_or_else(|| {
                usage(&program);
                eprintln!("ERROR: Spotify secret is not provided");
            })?;

            config.set_config(id, secret);

            config.save().map_err(|err| {
                eprintln!("ERROR: failed to save config: {err}");
            })?;

            println!("Credentials saved");

            Ok(())
        }
        "download" => {
            let url = args.next().ok_or_else(|| {
                usage(&program);
                eprintln!("ERROR: URL is not provided");
            })?;

            let (url_type, id) = parse_url(&url);

            let yt_dlp_path = Config::get_yt_dlp_path();
            let ffmpeg_path = Config::get_ffmpeg_path();

            match url_type {
                Spotify(spotify_type) => {
                    let access_token = AccessToken::load(&config).map_err(|err| {
                        eprintln!("ERROR: failed to load access token: {err}");
                    })?;

                    match spotify_type {
                        SpotifyType::Track => {
                            download_track(&access_token, &id, &yt_dlp_path, &ffmpeg_path);
                        }
                        SpotifyType::Album => {
                            download_album(&access_token, &id, &yt_dlp_path, &ffmpeg_path);
                        }
                        SpotifyType::Playlist => {
                            download_playlist(&access_token, &id, &yt_dlp_path, &ffmpeg_path);
                        }
                    }
                }
                SoundCloud(soundcloud_type) => match soundcloud_type {
                    SoundCloudType::Track => {
                        let mut command = Command::new(&yt_dlp_path);

                        command
                            .arg("-x")
                            .arg(&url)
                            .arg("--audio-format")
                            .arg("best");

                        command.arg("-o").arg("%(title)s.%(ext)s");

                        command.output().expect("Failed something while download");
                    }
                    SoundCloudType::Set => {
                        let mut command = Command::new(&yt_dlp_path);

                        command
                            .arg("-x")
                            .arg(&url)
                            .arg("--audio-format")
                            .arg("best");

                        command.arg("-o").arg(&format!("{}/%(title)s.%(ext)s", id));

                        command.output().expect("Failed something while download");
                    }
                },
            }
            Ok(())
        }
        _ => {
            usage(&program);
            eprintln!("ERROR: unknown subcommand {subcommand}");
            Err(())
        }
    }
}

fn main() -> ExitCode {
    check_dependencies().err();

    match entry() {
        Ok(()) => ExitCode::SUCCESS,
        Err(()) => ExitCode::FAILURE,
    }
}

fn check_dependencies() -> Result<(), String> {
    let mut err = String::new();

    if which("yt-dlp").is_err() {
        err.push_str("yt-dlp is not installed. ");
    }

    if which("ffmpeg").is_err() {
        err.push_str("ffmpeg is not installed. ");
    }

    if !err.is_empty() {
        eprintln!("ERROR: {}", err);
        return Err(err);
    }

    Ok(())
}
