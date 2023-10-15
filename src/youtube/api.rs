use serde_json::Value;

const DEFAULT_INNERTUBE_KEY: &str = "AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8";
#[derive(Debug, Clone)]
pub struct VideoInfo {
    pub id: String,
    pub title: String,
    pub duration_ms: usize,
}
pub fn search_videos(query: &str) -> Vec<VideoInfo> {
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

fn parse_duration_string(duration_str: &str) -> Result<usize, &'static str> {
    let parts: Vec<&str> = duration_str.split(':').collect();

    match parts.len() {
        2 => {
            // Format: "minutes:seconds"
            let minutes: usize = parts[0].parse().unwrap_or(0);
            let seconds: usize = parts[1].parse().unwrap_or(0);
            Ok(minutes * 60_000 + seconds * 1_000)
        }
        3 => {
            // Format: "hours:minutes:seconds"
            let hours: usize = parts[0].parse().unwrap_or(0);
            let minutes: usize = parts[1].parse().unwrap_or(0);
            let seconds: usize = parts[2].parse().unwrap_or(0);
            Ok(hours * 3_600_000 + minutes * 60_000 + seconds * 1_000)
        }
        _ => Err("Invalid duration format"),
    }
}
