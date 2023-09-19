use serde::Deserialize;
use url::Url;

#[derive(Debug)]
pub enum UrlType {
    Track,
    Playlist,
    Album,
}
#[derive(Debug, Deserialize)]
pub struct Track {
    name: String,
    artists: Vec<Artist>,
    album: Album,
}
#[derive(Debug, Deserialize)]
struct Artist {
    name: String,
}
#[derive(Debug, Deserialize)]
struct Album {
    name: String,
    release_date: String,
    artists: Vec<Artist>,
    images: Vec<Image>,
}
#[derive(Debug, Deserialize)]
struct Image {
    url: String,
    width: usize,
    height: usize,
}
pub fn parse_url(url: &str) -> (UrlType, String) {
    let parsed = Url::parse(url).unwrap_or_else(|err| {
        eprintln!("ERROR: Failed to parse URL: {err}");
        std::process::exit(1);
    });

    if parsed.host_str().expect("Failed to get host") != "open.spotify.com" {
        eprintln!("ERROR: Invalid host, only open.spotify.com is supported");
        std::process::exit(1);
    }

    let path = parsed.path();
    let mut parts = path.split('/');

    let _ = parts.next();
    let url_type = parts.next().expect("Failed to get URL type");
    let id = parts.next().expect("Failed to get ID");

    match url_type {
        "track" => (UrlType::Track, String::from(id)),
        "playlist" => (UrlType::Playlist, String::from(id)),
        "album" => (UrlType::Album, String::from(id)),
        _ => {
            eprintln!("ERROR: Invalid URL type, only track, playlist and album are supported");
            std::process::exit(1);
        },
    }
}
pub fn fetch_track(token: &str, id: &str) -> Track {
    let url = format!("https://api.spotify.com/v1/tracks/{}", id);

    let response = ureq::get(&url).set("Authorization", &format!("Bearer {}", token))
        .call()
        .unwrap_or_else(|err| {
            eprintln!("ERROR: Failed to make the request: {err}, check the URL");
            std::process::exit(1);
        });

    let body: Track = response.into_json().expect("Failed to parse JSON response");

    body
}