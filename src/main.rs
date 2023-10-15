use std::env;
use std::path::Path;
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
use parser::{
    parse_url, SoundCloudType, SpotifyType,
    UrlType::{SoundCloud, Spotify},
};
use spotify::access_token::AccessToken;

fn main() -> ExitCode {
    if check_dependencies().is_err() {
        return ExitCode::FAILURE;
    }

    match execute_command() {
        Ok(()) => ExitCode::SUCCESS,
        Err(()) => ExitCode::FAILURE,
    }
}

fn check_dependencies() -> Result<(), ()> {
    let missing_deps: Vec<&str> = vec!["yt-dlp", "ffmpeg"]
        .into_iter()
        .filter(|&dep| which(dep).is_err())
        .collect();

    if !missing_deps.is_empty() {
        eprintln!("ERROR: {} not installed.", missing_deps.join(" and "));
        return Err(());
    }

    Ok(())
}

fn execute_command() -> Result<(), ()> {
    let mut args = env::args();
    let _ = args.next().expect("Path to program is provided");

    let subcommand = args
        .next()
        .ok_or_else(|| handle_error("no subcommand is provided"))?;

    let mut config =
        Config::load().map_err(|err| handle_error(&format!("failed to load config: {err}")))?;

    match subcommand.as_str() {
        "config" => handle_config_command(&mut config, &mut args),
        "download" => handle_download_command(&config, &mut args),
        _ => {
            display_usage();
            eprintln!("ERROR: unknown subcommand {subcommand}");
            Err(())
        }
    }
}

fn handle_config_command(config: &mut Config, args: &mut env::Args) -> Result<(), ()> {
    let id = args
        .next()
        .ok_or_else(|| handle_error("Spotify client is not provided"))?;
    let secret = args
        .next()
        .ok_or_else(|| handle_error("Spotify secret is not provided"))?;
    let oauth_token = args.next().unwrap_or("".to_string());

    config.set_config(id, secret, oauth_token);

    config
        .save()
        .map_err(|err| handle_error(&format!("failed to save config: {err}")))?;

    println!("Credentials saved");

    Ok(())
}

fn handle_download_command(config: &Config, args: &mut env::Args) -> Result<(), ()> {
    // Extract and process download-related parameters
    let url = args.next().ok_or_else(|| {
        display_usage();
        eprintln!("ERROR: URL is not provided");
    })?;

    let (url_type, id) = parse_url(&url);

    let yt_dlp_path = Config::get_yt_dlp_path();
    let ffmpeg_path = Config::get_ffmpeg_path();

    match url_type {
        Spotify(spotify_type) => {
            handle_spotify_download(config, &spotify_type, &id, &yt_dlp_path, &ffmpeg_path)
        }
        SoundCloud(soundcloud_type) => {
            handle_soundcloud_download(config, &soundcloud_type, &url, &yt_dlp_path, &ffmpeg_path)
        }
    }

    Ok(())
}

fn handle_spotify_download(
    config: &Config,
    spotify_type: &SpotifyType,
    id: &str,
    yt_dlp_path: &Path,
    ffmpeg_path: &Path,
) {
    let access_token = AccessToken::load(config).map_err(|err| {
        eprintln!("ERROR: failed to load access token: {err}");
    });

    match access_token {
        Ok(token) => match spotify_type {
            SpotifyType::Track => {
                spotify::download::download_track(&token, id, yt_dlp_path, ffmpeg_path);
            }
            SpotifyType::Album => {
                spotify::download::download_album(&token, id, yt_dlp_path, ffmpeg_path);
            }
            SpotifyType::Playlist => {
                spotify::download::download_playlist(&token, id, yt_dlp_path, ffmpeg_path);
            }
        },
        Err(_) => {
            eprintln!("ERROR: Access token not found or invalid.");
        }
    }
}

fn handle_soundcloud_download(
    config: &Config,
    soundcloud_type: &SoundCloudType,
    url: &str,
    yt_dlp_path: &Path,
    ffmpeg_path: &Path,
) {
    let oauth_token = config.get_soundcloud_oauth_token();

    if oauth_token.is_empty() {
        display_usage();
        eprintln!("ERROR: Soundcloud OAuth token not provided, set the config again");
        return;
    }

    match soundcloud_type {
        SoundCloudType::Track => {
            soundcloud::download::download_track(oauth_token, url, yt_dlp_path, ffmpeg_path);
        }
        SoundCloudType::Set => {
            soundcloud::download::download_set(oauth_token, url, yt_dlp_path, ffmpeg_path);
        }
    }
}

fn handle_error(message: &str) {
    eprintln!("ERROR: {}", message);
    display_usage();
    exit(1);
}

fn display_usage() {
    eprintln!("Usage: spotydw [SUBCOMMAND] [OPTIONS]");
    eprintln!("Subcommands:");
    eprintln!("     config <SPOTIFY_CLIENT_ID> <SPOTIFY_CLIENT_SECRET> [SOUNDCLOUD_OAUTH_TOKEN]");
    eprintln!("     download <URL>");
}
