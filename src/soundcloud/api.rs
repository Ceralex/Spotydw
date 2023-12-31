use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Track {
    pub title: String,
    pub id: u64,
    pub user: User,
    pub display_date: String,
    pub artwork_url: String,
    pub permalink_url: String,
}
#[derive(Debug, Deserialize)]
pub struct User {
    pub username: String,
}
pub fn fetch_track(oauth_token: &str, url: &str) -> Track {
    let url = format!("https://api-v2.soundcloud.com/resolve?url={}", url);

    let response = ureq::get(&url)
        .set("Authorization", &format!("OAuth {}", oauth_token))
        .call()
        .unwrap_or_else(|err| {
            eprintln!("ERROR: Failed to make the request: {err}, check the URL");
            std::process::exit(1);
        });

    let body: Track = response.into_json().expect("Failed to parse JSON response");

    body
}

#[derive(Debug, Deserialize)]
pub struct Set {
    pub title: String,
    pub tracks: Vec<SetTrack>,
}
#[derive(Debug, Deserialize)]
pub struct SetTrack {
    pub id: u64,
}
pub fn fetch_set(oauth_token: &str, url: &str) -> Set {
    let url = format!("https://api-v2.soundcloud.com/resolve?url={}", url);

    let response = ureq::get(&url)
        .set("Authorization", &format!("OAuth {}", oauth_token))
        .call()
        .unwrap_or_else(|err| {
            eprintln!("ERROR: Failed to make the request: {err}, check the URL");
            std::process::exit(1);
        });

    let body: Set = response.into_json().expect("Failed to parse JSON response");

    body
}
pub fn fetch_set_track(oauth_token: &str, id: u64) -> Track {
    let url = format!("https://api-v2.soundcloud.com/tracks/{}", id);

    let response = ureq::get(&url)
        .set("Authorization", &format!("OAuth {}", oauth_token))
        .call()
        .unwrap_or_else(|err| {
            eprintln!("ERROR: Failed to make the request: {err}, check the URL");
            std::process::exit(1);
        });

    let body: Track = response.into_json().expect("Failed to parse JSON response");

    body
}
