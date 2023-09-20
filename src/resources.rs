use std::{env, process};
use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::ops::Add;
use std::path::PathBuf;
use std::process::Command;
use crate::config::Config;

pub fn check_dependencies() {
    let config_dir = Config::config_path().unwrap();

    let resource_dir = config_dir.join("resources");

    if !resource_dir.exists() {
        fs::create_dir_all(&resource_dir).unwrap_or_else(|err| {
            eprintln!("ERROR: Failed to create directory: {err}");
            process::exit(1);
        });
        download_ytdlp(&resource_dir).unwrap_or_else(|err| {
            eprintln!("ERROR: Failed to download yt-dlp: {err}");
            process::exit(1);
        });
    }
}

pub fn download_ytdlp(resource_dir: &PathBuf) -> Result<(), ureq::Error> {
    let os = env::consts::OS;

    let base_url = String::from("https://github.com/yt-dlp/yt-dlp/releases/latest/download/");
    let file_name = match os {
        "linux" => "yt-dlp_linux",
        "windows" => "yt-dlp.exe",
        "macos" => "yt-dlp_macos",
        _ => {
            eprintln!("ERROR: Unsupported OS: {}", os);
            process::exit(1);
        }
    };

    println!("Downloading yt-dlp for {}...", os);

    let url = base_url.add(file_name);

    let response = ureq::get(&url).call()?;

    if response.status() == 200 {
        let file = File::create(&resource_dir).unwrap_or_else(|err| {
            eprintln!("ERROR: Failed to create file: {err}");
            process::exit(1);
        });

        let mut buf = BufWriter::new(&file);
        std::io::copy(&mut response.into_reader(), &mut buf).unwrap();

        if os != "windows" {
            Command::new("chmod")
                .arg("+x")
                .arg(&resource_dir)
                .spawn()
                .expect("ERROR: Failed to set permissions");
        } else {
            Command::new("icacls")
                .arg(&resource_dir)
                .arg("/grant")
                .arg("Everyone:F")
                .spawn()
                .expect("ERROR: Failed to set permissions");
        }
    } else {
        eprintln!("ERROR: Failed to download yt-dlp");
        process::exit(1);
    }
    Ok(())
}