use crate::{ffmpeg, soundcloud};
use ffmpeg::Metadata;
use rayon::prelude::*;
use soundcloud::api::{fetch_set, fetch_set_track, fetch_track};
use std::path::Path;
use std::process::Command;

pub fn download_track(oauth_token: &str, url: &str, yt_dlp_path: &Path, ffmpeg_path: &Path) {
    let track = fetch_track(oauth_token, url);

    println!("Downloading track: {}", track.title);

    let mut command = Command::new(yt_dlp_path);

    command.arg("-x").arg(url).arg("--audio-format").arg("best");

    command.arg("-o").arg("%(id)s.%(ext)s");

    command.output().expect("Failed something while download");

    let input_file = format!("{}.mp3", &track.id);
    let input_file_path = Path::new(&input_file);

    let metadata = Metadata {
        title: track.title.clone(),
        artists: vec![track.user.username.clone()],
        album_artists: vec![track.user.username.clone()],
        album_name: track.title.clone(),
        total_tracks: 1,
        track_number: 1,
        release_date: track.display_date.clone(),
        album_cover_url: track.artwork_url.clone(),
    };

    let output_file = format!(
        "{}.mp3",
        track
            .title
            .replace(['<', '>', ':', '"', '/', '\\', '|', '?', '*'], " ")
    );
    let output_file_path = Path::new(&output_file);

    ffmpeg::process_with_metadata(ffmpeg_path, input_file_path, output_file_path, &metadata);
}

pub fn download_set(oauth_token: &str, url: &str, yt_dlp_path: &Path, ffmpeg_path: &Path) {
    let set = fetch_set(oauth_token, url);

    println!("Downloading set: {}", set.title);

    set.tracks
        .par_iter()
        .enumerate()
        .for_each(|(index, track)| {
            let track = fetch_set_track(oauth_token, track.id);

            let mut command = Command::new(yt_dlp_path);

            command
                .arg("-x")
                .arg(&track.permalink_url)
                .arg("--audio-format")
                .arg("best");

            let folder_path = set
                .title
                .replace(['<', '>', ':', '"', '/', '\\', '|', '?', '*'], " ");

            command
                .arg("-o")
                .arg(&format!("{}/%(id)s.%(ext)s", folder_path));

            command.output().expect("Failed something while download");

            let input_file = format!("{}/{}.mp3", folder_path, &track.id);
            let input_file_path = Path::new(&input_file);

            let metadata = Metadata {
                title: track.title.clone(),
                artists: vec![track.user.username.clone()],
                album_artists: vec![track.user.username.clone()],
                album_name: set.title.clone(),
                total_tracks: set.tracks.len(),
                track_number: index + 1,
                release_date: track.display_date.clone(),
                album_cover_url: track.artwork_url.clone(),
            };

            let output_file = format!(
                "{}/{}.mp3",
                folder_path,
                track
                    .title
                    .replace(['<', '>', ':', '"', '/', '\\', '|', '?', '*'], " ")
            );
            let output_file_path = Path::new(&output_file);

            ffmpeg::process_with_metadata(
                ffmpeg_path,
                input_file_path,
                output_file_path,
                &metadata,
            );
        });
}
