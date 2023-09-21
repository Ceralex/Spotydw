use std::{env, fs};
use std::path::PathBuf;
use std::process::{Command, ExitCode};

mod config;
use config::Config;

mod spotify {
    pub mod access_token;
    pub mod api;
}
use spotify::access_token::AccessToken;
use spotify::api::{fetch_track, UrlType};

mod youtube;

use which::which;
use crate::spotify::api::Track;

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

            let access_token = AccessToken::load(&config).map_err(|err| {
                eprintln!("ERROR: failed to load access token: {err}");
            })?;

            let (url_type, id) = spotify::api::parse_url(&url);

            let yt_dlp_path = Config::get_yt_dlp_path();
            let ffmpeg_path = Config::get_ffmpeg_path();

            match url_type {
                UrlType::Track => {
                    let track = fetch_track(access_token.get_token(), &id);

                    let qry = format!(
                        "{} - {}",
                        track.name,
                        track
                            .artists
                            .iter()
                            .map(|artist| artist.name.clone())
                            .collect::<Vec<String>>()
                            .join(", ")
                    );

                    let videos = youtube::search(&qry);

                    let video = videos.first().expect("No videos found");

                    let input_file =
                        youtube::download(&yt_dlp_path, &video, None).map_err(|err| {
                            eprintln!("ERROR: failed to download video: {err}");
                        })?;

                    metadata_and_to_mp3(&ffmpeg_path, &input_file, &track);
                }
                UrlType::Playlist => {
                    unimplemented!("Playlist download")
                }
                UrlType::Album => {
                    unimplemented!("Album download")
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

fn metadata_and_to_mp3(ffmpeg_path: &PathBuf, input_file: &String, t: &Track) {
    let mut command = Command::new(&ffmpeg_path);

    let artists = t
        .artists
        .iter()
        .map(|artist| artist.name.clone())
        .collect::<Vec<String>>()
        .join("; ");
    let album_artists = t
        .album
        .artists
        .iter()
        .map(|artist| artist.name.clone())
        .collect::<Vec<String>>()
        .join("; ");

    command
        .args(&[
            "-i", &input_file,
            "-i", &t.album.images[0].url,
            "-metadata", &format!("title={}", t.name),
            "-metadata", &format!("artist={}", artists),
            "-metadata", &format!("album_artist={}", album_artists),
            "-metadata", &format!("album={}", t.album.name),
            "-metadata", &format!("date={}", t.album.release_date),
            "-map", "0",
            "-map", "1",
            "-c:v", "mjpeg",
            "-q:v", "2",
            "-c:a", "libmp3lame",
            "-q:a", "4",
            "-id3v2_version", "3",
            "-metadata:s:v", "title='Album cover'",
            "-metadata:s:v", "comment='Cover (front)'",
            &format!("{}.mp3", t.name),
        ]);

    let output = command.output().expect("Failed to execute ffmpeg");

    if !output.status.success() {
        eprintln!(
            "ERROR: ffmpeg failed with exit code {}",
            output.status.code().unwrap_or(1)
        );
        std::process::exit(1);
    }

    fs::remove_file(&input_file).expect("Failed to remove input file");
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
