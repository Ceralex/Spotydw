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
    pub next: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PlaylistTrack {
    pub track: Track,
}

#[derive(Debug, Deserialize)]
pub struct PlaylistNameResponse {
    name: String,
}
pub fn fetch_playlist(token: &str, id: &str) -> Playlist {
    let url = format!("https://api.spotify.com/v1/playlists/{id}?fields=name");
    let name_response = ureq::get(&url)
        .set("Authorization", &format!("Bearer {}", token))
        .call()
        .unwrap_or_else(|err| {
            eprintln!("ERROR: Failed to make the request: {err}, check the URL");
            std::process::exit(1);
        }).into_json::<PlaylistNameResponse>().expect("Failed to get playlist name");

    let mut playlist = Playlist {
        name: name_response.name,
        tracks: Items {
            items: Vec::new(),
            next: Some(String::new()),
        }
    };

    let mut offset = 0;

    loop {
        let url = format!(
            "https://api.spotify.com/v1/playlists/{}/tracks?offset={}&limit=100&fields=next,items.track(name,artists.name,duration_ms,track_number,album(name,release_date,artists,images,total_tracks))",
            id, offset
        );

        let response = ureq::get(&url)
            .set("Authorization", &format!("Bearer {}", token))
            .call()
            .unwrap_or_else(|err| {
                eprintln!("ERROR: Failed to make the request: {err}, check the URL");
                std::process::exit(1);
            });

        let body: Items = response.into_json().expect("Failed to get playlist tracks");

        playlist.tracks.items.extend_from_slice(&body.items);

        offset += 100;

        if body.next.unwrap_or("".to_string()).is_empty() {
            break
        }
    }

    playlist
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
