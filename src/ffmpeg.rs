use crate::spotify::api::Track;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub fn metadata_and_to_mp3(ffmpeg_path: &PathBuf, input_file: &PathBuf, t: &Track) {
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

    let mut output_file_path = PathBuf::new();
    if let Some(parent) = input_file.parent() {
        output_file_path.push(parent);
    }
    output_file_path.push(format!("{}.mp3", t.name.replace("/", "-")));

    command.args(&[
        "-i",
        &input_file.to_str().unwrap(),
        "-i",
        &t.album.images[0].url,
        "-metadata",
        &format!("title={}", t.name),
        "-metadata",
        &format!("artist={}", artists),
        "-metadata",
        &format!("album_artist={}", album_artists),
        "-metadata",
        &format!("album={}", t.album.name),
        "-metadata",
        &format!("date={}", t.album.release_date),
        "-map",
        "0",
        "-map",
        "1",
        "-c:v",
        "mjpeg",
        "-q:v",
        "2",
        "-c:a",
        "libmp3lame",
        "-q:a",
        "4",
        "-id3v2_version",
        "3",
        "-metadata:s:v",
        "title='Album cover'",
        "-metadata:s:v",
        "comment='Cover (front)'",
        "-y",
        &output_file_path.to_str().unwrap(),
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

    fs::remove_file(&input_file).expect("Failed to remove input file");

    println!("{} downloaded", t.name);
}
