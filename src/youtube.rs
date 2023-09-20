use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;

#[derive(Debug)]
pub struct VideoInfo {
    pub id: String,
    pub title: String,
    pub duration: Duration,
}
pub fn search(ytdlp_path: &PathBuf, query: &str) -> Vec<VideoInfo> {
    let output = Command::new(ytdlp_path)
        .arg(format!("ytsearch:'{}'", &query))
        .arg("--get-id")
        .arg("--get-title")
        .arg("--get-duration")
        .output()
        .expect("Failed to execute yt-dlp command");

    let videos = String::from_utf8(output.stdout)
        .map(|output_str| {
            output_str.trim().split('\n').collect::<Vec<_>>()
                .chunks(3)
                .map(|chunk| {
                    VideoInfo {
                        id: chunk[0].to_string(),
                        title: chunk[1].to_string(),
                        duration: parse_duration_string(chunk[2])
                    }
                })
                .collect()
        })
        .unwrap_or_else(|_| Vec::new());

    videos
}

fn parse_duration_string(duration_str: &str) -> Duration {
    let parts: Vec<&str> = duration_str.split(':').collect();

    // Parse hours and minutes
    let minutes: u64 = parts[0].parse().unwrap_or(0);
    let seconds: u64 = parts[1].parse().unwrap_or(0);

    // Calculate the total duration in seconds
    let total_seconds = (minutes * 60) + seconds;

    Duration::from_secs(total_seconds)
}