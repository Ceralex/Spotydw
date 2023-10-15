use std::fs;
use std::path::Path;
use std::process::Command;

pub struct Metadata {
    pub title: String,
    pub artists: Vec<String>,
    pub album_artists: Vec<String>,
    pub album_name: String,
    pub total_tracks: usize,
    pub track_number: usize,
    pub release_date: String,
    pub album_cover_url: String,
}

pub fn process_with_metadata(
    ffmpeg_path: &Path,
    input_file: &Path,
    output_file: &Path,
    metadata: &Metadata,
) {
    let mut command = Command::new(ffmpeg_path);

    let artists = metadata.artists.join("; ");
    let album_artists = metadata.album_artists.join("; ");

    command.args([
        "-i",
        input_file.to_str().unwrap(),
        "-f",
        "jpeg_pipe",
        "-i",
        &metadata.album_cover_url,
        "-metadata",
        &format!("title={}", metadata.title),
        "-metadata",
        &format!("artist={}", artists),
        "-metadata",
        &format!("album_artist={}", album_artists),
        "-metadata",
        &format!("album={}", metadata.album_name),
        "-metadata",
        &format!("track={}/{}", metadata.track_number, metadata.total_tracks),
        "-metadata",
        &format!("date={}", metadata.release_date),
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
        "-metadata:s:v",
        "title='Album cover'",
        "-metadata:s:v",
        "comment='Cover (front)'",
        "-y",
        output_file.to_str().unwrap(),
    ]);

    let output = command.output().expect("Failed to execute ffmpeg");

    if !output.status.success() {
        eprintln!(
            "ERROR: ffmpeg failed with exit code {}",
            output.status.code().unwrap_or(1)
        );
        eprintln!("ffmpeg output: {}", String::from_utf8_lossy(&output.stderr));
    }

    println!("{} downloaded", metadata.title);
    fs::remove_file(input_file).unwrap();
}
