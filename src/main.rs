use std::env;
use std::process::{exit, ExitCode};
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
mod soundcloud {
    pub mod api;
    pub mod download;
}

use config::Config;
use parser::parse_url;
use parser::UrlType::{SoundCloud, Spotify};
use parser::{SoundCloudType, SpotifyType};
use spotify::access_token::AccessToken;

// TODO: Find a better method to set config, you should be able to set spotify credentials and soundcloud separated
fn usage(program: &str) {
    eprintln!("Usage: {program} [SUBCOMMAND] [OPTIONS]");
    eprintln!("Subcommands:");
    eprintln!("     config <SPOTIFY_CLIENT_ID> <SPOTIFY_CLIENT_SECRET> [SOUNDCLOUD_OAUTH_TOKEN]");
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

            let oauth_token = args.next().unwrap_or("".to_string());
            config.set_config(id, secret, oauth_token);

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
                            spotify::download::download_track(
                                &access_token,
                                &id,
                                &yt_dlp_path,
                                &ffmpeg_path,
                            );
                        }
                        SpotifyType::Album => {
                            spotify::download::download_album(
                                &access_token,
                                &id,
                                &yt_dlp_path,
                                &ffmpeg_path,
                            );
                        }
                        SpotifyType::Playlist => {
                            spotify::download::download_playlist(
                                &access_token,
                                &id,
                                &yt_dlp_path,
                                &ffmpeg_path,
                            );
                        }
                    }
                }
                SoundCloud(soundcloud_type) => {
                    let oauth_token = config.get_soundcloud_oauth_token();

                    if oauth_token.is_empty() {
                        usage(&program);
                        eprintln!(
                            "ERROR: Soundcloud OAuth token not provided, set the config again"
                        );
                        exit(1);
                    }
                    match soundcloud_type {
                        SoundCloudType::Track => soundcloud::download::download_track(
                            config.get_soundcloud_oauth_token(),
                            &url,
                            &yt_dlp_path,
                            &ffmpeg_path,
                        ),
                        SoundCloudType::Set => soundcloud::download::download_set(
                            config.get_soundcloud_oauth_token(),
                            &url,
                            &yt_dlp_path,
                            &ffmpeg_path,
                        ),
                    }
                }
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
