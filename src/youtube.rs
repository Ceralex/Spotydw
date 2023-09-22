use serde_json::Value;
use std::path::{Path, PathBuf};
use std::process::Command;

const DEFAULT_INNERTUBE_KEY: &str = "AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8";
#[derive(Debug, Clone)]
pub struct VideoInfo {
    pub id: String,
    pub title: String,
    pub duration_ms: u64,
}
pub fn search(query: &str) -> Vec<VideoInfo> {
    let body = format!(
        r#"
{{
  "query": "{query}",
  "params": "EAgIQAQ%253D%253D",
  "context": {{
    "client": {{
      "clientName": "WEB",
      "clientVersion": "1.20220406.00.00",
    }}
  }}
}}
"#
    );

    let url = format!(
        "https://youtube.com/youtubei/v1/search?key={}",
        DEFAULT_INNERTUBE_KEY
    );
    let response = ureq::post(&url)
        .set("Content-Type", "application/json")
        .set("Host", "www.youtube.com")
        .set("Referer", "https://www.youtube.com")
        .send(body.as_bytes())
        .unwrap();

    let data: Value = response.into_json().unwrap();

    let mut videos = Vec::new();

    let results = data["contents"]["twoColumnSearchResultsRenderer"]["primaryContents"]
        ["sectionListRenderer"]["contents"][0]["itemSectionRenderer"]["contents"]
        .as_array()
        .unwrap();

    for item in results {
        let id = item["videoRenderer"]["videoId"].as_str();
        let title = item["videoRenderer"]["title"]["runs"][0]["text"].as_str();
        let duration = item["videoRenderer"]["lengthText"]["simpleText"].as_str();

        if id.is_none() || title.is_none() || duration.is_none() {
            continue;
        }

        videos.push(VideoInfo {
            id: id.unwrap().to_string(),
            title: title.unwrap().to_string(),
            duration_ms: parse_duration_string(duration.unwrap()).expect("Duration converted"),
        });
    }
    videos
}

fn parse_duration_string(duration_str: &str) -> Result<u64, &'static str> {
    let parts: Vec<&str> = duration_str.split(':').collect();

    match parts.len() {
        2 => {
            // Format: "minutes:seconds"
            let minutes: u64 = parts[0].parse().unwrap_or(0);
            let seconds: u64 = parts[1].parse().unwrap_or(0);
            Ok(minutes * 60_000 + seconds * 1_000)
        }
        3 => {
            // Format: "hours:minutes:seconds"
            let hours: u64 = parts[0].parse().unwrap_or(0);
            let minutes: u64 = parts[1].parse().unwrap_or(0);
            let seconds: u64 = parts[2].parse().unwrap_or(0);
            Ok(hours * 3_600_000 + minutes * 60_000 + seconds * 1_000)
        }
        _ => Err("Invalid duration format"),
    }
}

pub fn download(
    yt_dlp_path: &Path,
    video: &VideoInfo,
    output_path: Option<&Path>,
) -> Result<PathBuf, std::io::Error> {
    let url = format!("https://www.youtube.com/watch?v={}", video.id);

    let mut command = Command::new(&yt_dlp_path);

    command
        .arg("-x")
        .arg(&url)
        .arg("--audio-format")
        .arg("best");

    // If output_path is a provided, use it
    let command = if output_path.is_some() {
        command.arg("-o").arg(&format!(
            "{}/%(id)s.%(ext)s",
            output_path.unwrap().to_str().unwrap()
        ))
    } else {
        command.arg("-o").arg("%(id)s.%(ext)s")
    };

    let output = command.output()?;

    if !output.status.success() {
        eprintln!(
            "ERROR: yt-dlp failed with exit code {}",
            output.status.code().unwrap_or(1)
        );
        eprintln!("yt-dlp output: {}", String::from_utf8_lossy(&output.stderr));
    }

    let output_file = if let Some(output_path) = output_path {
        output_path.join(format!("{}.opus", video.id))
    } else {
        PathBuf::from(&format!("{}.opus", video.id))
    };

    Ok(output_file)
}
