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
    pub name: String,
    pub artists: Vec<Artist>,
    pub album: Album,
}
#[derive(Debug, Deserialize)]
pub struct Artist {
    pub name: String,
}
#[derive(Debug, Deserialize)]
pub struct Album {
    pub name: String,
    pub release_date: String,
    pub artists: Vec<Artist>,
    pub images: Vec<Image>,
}
#[derive(Debug, Deserialize)]
pub struct Image {
    pub url: String,
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
    let type_string = parts.next().expect("Failed to get URL type");
    let id = parts.next().expect("Failed to get ID");

    let url_type = match type_string {
        "track" => UrlType::Track,
        "playlist" => UrlType::Playlist,
        "album" => UrlType::Album,
        _ => {
            eprintln!("ERROR: Invalid URL type, only track, playlist and album are supported");
            std::process::exit(1);
        },
    };

    (url_type, id.to_string())
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