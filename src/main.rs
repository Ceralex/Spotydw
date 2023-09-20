use std::env;
use std::process::ExitCode;

mod config;
use config::Config;

mod resources;
use crate::resources::check_dependencies;

mod spotify {
    pub mod access_token;
    pub mod api;
}
use spotify::access_token::AccessToken;
use spotify::api::{UrlType, fetch_track};

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

            check_dependencies();

            let (url_type, id) = spotify::api::parse_url(&url);

            match url_type {
                UrlType::Track => {
                    let track = fetch_track(access_token.get_token(), &id);

                    let _qry = format!("{} - {}", track.name, track.artists.iter().map(|artist| artist.name.clone()).collect::<Vec<String>>().join(", "));
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

fn main() -> ExitCode {
    match entry() {
        Ok(()) => ExitCode::SUCCESS,
        Err(()) => ExitCode::FAILURE,
    }
}
