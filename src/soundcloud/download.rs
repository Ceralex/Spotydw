use crate::soundcloud;
use crate::soundcloud::api::fetch_set_track;
use rayon::prelude::*;
use soundcloud::api::{fetch_set, fetch_track};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub fn download_track(
    oauth_token: &str,
    url: &String,
    yt_dlp_path: &PathBuf,
    ffmpeg_path: &PathBuf,
) {
    let track = fetch_track(oauth_token, url);

    println!("Downloading track: {}", track.title);

    let mut command = Command::new(&yt_dlp_path);

    command
        .arg("-x")
        .arg(&url)
        .arg("--audio-format")
        .arg("best");

    command.arg("-o").arg(&format!("%(id)s.%(ext)s"));

    command.output().expect("Failed something while download");

    let mut command = Command::new(&ffmpeg_path);

    let input_file_path = format!("{}.mp3", &track.id);
    let output_file_path = format!(
        "{}.mp3",
        track
            .title
            .replace(&['<', '>', ':', '"', '/', '\\', '|', '?', '*'], " ")
    );

    command.args(&[
        "-i",
        &input_file_path,
        "-f",
        "jpeg_pipe",
        "-i",
        &track.artwork_url,
        "-metadata",
        &format!("title={}", track.title),
        "-metadata",
        &format!("artist={}", track.user.username),
        "-metadata",
        &format!("album_artist={}", track.user.username),
        "-metadata",
        &format!("album={}", track.title),
        "-metadata",
        &format!("track=1/1"),
        "-metadata",
        &format!("date={}", track.display_date),
        "-c",
        "copy",
        "-map",
        "0",
        "-map",
        "1",
        "-metadata:s:v",
        "title='Album cover'",
        "-metadata:s:v",
        "comment='Cover (front)'",
        "-y",
        &output_file_path,
    ]);
    let output = command.output().expect("Failed to execute ffmpeg");

    if !output.status.success() {
        eprintln!(
            "ERROR: ffmpeg failed with exit code {}",
            output.status.code().unwrap_or(1)
        );
        eprintln!("ffmpeg output: {}", String::from_utf8_lossy(&output.stderr));

        return;
    }

    fs::remove_file(&input_file_path).expect("Failed to remove input file");

    println!("{} downloaded", track.title);
}

pub fn download_set(oauth_token: &str, url: &String, yt_dlp_path: &PathBuf, ffmpeg_path: &PathBuf) {
    let set = fetch_set(oauth_token, url);

    println!("Downloading set: {}", set.title);

    set.tracks.par_iter().for_each(|track| {
        let track = fetch_set_track(oauth_token, track.id);

        let mut command = Command::new(&yt_dlp_path);

        command
            .arg("-x")
            .arg(&track.permalink_url)
            .arg("--audio-format")
            .arg("best");

        command
            .arg("-o")
            .arg(&format!("{}/%(id)s.%(ext)s", &set.title));

        command.output().expect("Failed something while download");

        let mut command = Command::new(&ffmpeg_path);

        let input_file_path = format!("{}/{}.mp3", &set.title, &track.id);
        let output_file_path = format!(
            "{}/{}.mp3",
            &set.title,
            track
                .title
                .replace(&['<', '>', ':', '"', '/', '\\', '|', '?', '*'], " ")
        );

        command.args(&[
            "-i",
            &input_file_path,
            "-f",
            "jpeg_pipe",
            "-i",
            &track.artwork_url,
            "-metadata",
            &format!("title={}", track.title),
            "-metadata",
            &format!("artist={}", track.user.username),
            "-metadata",
            &format!("album_artist={}", track.user.username),
            "-metadata",
            &format!("album={}", set.title),
            "-metadata",
            &format!("track=1/1"),
            "-metadata",
            &format!("date={}", track.display_date),
            "-c",
            "copy",
            "-map",
            "0",
            "-map",
            "1",
            "-metadata:s:v",
            "title='Album cover'",
            "-metadata:s:v",
            "comment='Cover (front)'",
            "-y",
            &output_file_path,
        ]);
        let output = command.output().expect("Failed to execute ffmpeg");

        if !output.status.success() {
            eprintln!(
                "ERROR: ffmpeg failed with exit code {}",
                output.status.code().unwrap_or(1)
            );
            eprintln!("ffmpeg output: {}", String::from_utf8_lossy(&output.stderr));

            return;
        }

        fs::remove_file(&input_file_path).expect("Failed to remove input file");

        println!("{} downloaded", track.title);
    });
}
