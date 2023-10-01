use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Track {
    pub name: String,
    pub artists: Vec<Artist>,
    pub album: Album,
    pub track_number: u64,
    pub duration_ms: u64,
}
#[derive(Debug, Deserialize, Clone)]
pub struct Artist {
    pub name: String,
}
#[derive(Debug, Deserialize, Clone)]
pub struct Album {
    pub name: String,
    pub release_date: String,
    pub total_tracks: u64,
    pub artists: Vec<Artist>,
    pub images: Vec<Image>,
}
#[derive(Debug, Deserialize, Clone)]
pub struct Image {
    pub url: String,
}

pub fn fetch_track(token: &str, id: &str) -> Track {
    let url = format!("https://api.spotify.com/v1/tracks/{}", id);

    let response = ureq::get(&url)
        .set("Authorization", &format!("Bearer {}", token))
        .call()
        .unwrap_or_else(|err| {
            eprintln!("ERROR: Failed to make the request: {err}, check the URL");
            std::process::exit(1);
        });

    let body: Track = response.into_json().expect("Failed to parse JSON response");

    body
}

#[derive(Debug, Deserialize)]
pub struct Playlist {
    pub name: String,
    pub tracks: Items,
}

#[derive(Debug, Deserialize)]
pub struct Items {
    pub items: Vec<PlaylistTrack>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PlaylistTrack {
    pub track: Track,
}
pub fn fetch_playlist(token: &str, id: &str) -> Playlist {
    // name,tracks.items.track(name,artists.name,duration_ms,track_number,album(name,release_date,artists,images,total_tracks))
    let url = format!("https://api.spotify.com/v1/playlists/{id}?fields=name%2Ctracks.items.track%28name%2Cartists.name%2Cduration_ms%2Ctrack_number%2Calbum%28name%2Crelease_date%2Cartists%2Cimages%2Ctotal_tracks%29%29");

    let response = ureq::get(&url)
        .set("Authorization", &format!("Bearer {}", token))
        .call()
        .unwrap_or_else(|err| {
            eprintln!("ERROR: Failed to make the request: {err}, check the URL");
            std::process::exit(1);
        });

    let body: Playlist = response.into_json().expect("Failed to parse JSON response");

    body
}

#[derive(Debug, Deserialize)]
pub struct AlbumResponse {
    pub name: String,
    pub release_date: String,
    pub artists: Vec<Artist>,
    pub images: Vec<Image>,
    pub tracks: AlbumItems,
}
#[derive(Debug, Deserialize)]
pub struct AlbumItems {
    pub items: Vec<AlbumTrack>,
    pub total: u64,
}
#[derive(Debug, Deserialize)]
pub struct AlbumTrack {
    pub name: String,
    pub artists: Vec<Artist>,
    pub track_number: u64,
    pub duration_ms: u64,
}
pub fn fetch_album(token: &str, id: &str) -> AlbumResponse {
    let url = format!("https://api.spotify.com/v1/albums/{id}");

    let response = ureq::get(&url)
        .set("Authorization", &format!("Bearer {}", token))
        .call()
        .unwrap_or_else(|err| {
            eprintln!("ERROR: Failed to make the request: {err}, check the URL");
            std::process::exit(1);
        });

    let body: AlbumResponse = response.into_json().expect("Failed to parse JSON response");

    body
}
