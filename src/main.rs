use std::env;
use std::process::ExitCode;

mod config;
use config::Config;

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
            println!("Setting secret");

            let id = args.next().ok_or_else(|| {
                usage(&program);
                eprintln!("ERROR: Spotify client is not provided");
            })?;
            let secret = args.next().ok_or_else(|| {
                usage(&program);
                eprintln!("ERROR: Spotify secret is not provided");
            })?;

            config.set(id, secret);
            config.save().map_err(|err| {
                eprintln!("ERROR: failed to save config: {err}");
            })?;

            Ok(())
        }
        "download" => {
            println!("Downloading");
            println!("Config: {:?}", config);

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
