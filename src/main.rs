use std::env;
use std::path::Path;
use std::process::ExitCode;
use which::which;
use rayon::prelude::*;

mod config;
use config::Config;

mod spotify {
    pub mod access_token;
    pub mod api;
}
use spotify::access_token::AccessToken;
use spotify::api::{fetch_track, UrlType};
use crate::spotify::api::fetch_playlist;

mod youtube;

mod ffmpeg;

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

                    println!("Downloading track: {}", track.name);

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

                    // Take the video in the first 5 results with the most similar duration as the track
                    let video = videos
                        .iter()
                        .take(5)
                        .min_by_key(|&video| video.duration_ms.wrapping_sub(track.duration_ms))
                        .unwrap();

                    let input_file =
                        youtube::download(&yt_dlp_path, &video, None).map_err(|err| {
                            eprintln!("ERROR: failed to download video: {err}");
                        })?;

                    ffmpeg::metadata_and_to_mp3(&ffmpeg_path, &input_file, &track);
                }
                UrlType::Playlist => {
                    let playlist = fetch_playlist(access_token.get_token(), &id);

                    println!("Downloading playlist: {}", playlist.name);

                    playlist.tracks.items.par_iter().for_each(|playlist_track| {
                        let track = &playlist_track.track;

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

                        // Take the video in the first 5 results with the most similar duration as the track
                        let video = videos
                            .iter()
                            .take(5)
                            .min_by_key(|&video| video.duration_ms.wrapping_sub(track.duration_ms))
                            .unwrap();

                        let input_file =
                            youtube::download(&yt_dlp_path, &video, Some(Path::new(&playlist.name))).map_err(|err| {
                                eprintln!("ERROR: failed to download video: {err}");
                            }).unwrap();

                        ffmpeg::metadata_and_to_mp3(&ffmpeg_path, &input_file, &track);
                    });
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
