use std::path::{Path, PathBuf};
use std::process::Command;

use crate::youtube;
use youtube::api::VideoInfo;

pub fn download_video(
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
        .arg("opus");

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
