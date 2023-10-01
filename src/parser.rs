use url::Url;

#[derive(Debug)]
pub enum UrlType {
    Spotify(SpotifyType),
    SoundCloud(SoundCloudType),
}

#[derive(Debug)]
pub enum SpotifyType {
    Track,
    Album,
    Playlist,
}

#[derive(Debug)]
pub enum SoundCloudType {
    Track,
    Set,
}

pub fn parse_url(url: &str) -> (UrlType, String) {
    let parsed = Url::parse(url).unwrap_or_else(|err| {
        eprintln!("ERROR: Failed to parse URL: {}", err);
        std::process::exit(1);
    });

    let host = parsed.host_str().expect("Failed to get host");

    if host == "open.spotify.com" {
        let path = parsed.path();
        let mut parts = path.split('/');

        let _ = parts.next();
        let type_string = parts.next().expect("Failed to get URL type");
        let id = parts.next().expect("Failed to get ID");

        let url_type = match type_string {
            "track" => UrlType::Spotify(SpotifyType::Track),
            "playlist" => UrlType::Spotify(SpotifyType::Playlist),
            "album" => UrlType::Spotify(SpotifyType::Album),
            _ => {
                eprintln!("ERROR: Invalid Spotify URL type, only track, playlist, and album are supported");
                std::process::exit(1);
            }
        };

        return (url_type, id.to_string());
    } else if host == "soundcloud.com" {
        let path = parsed.path();
        let mut parts = path.split('/');

        let _ = parts.next();
        let _user = parts.next();

        let url_type = match parts.next() {
            Some("sets") => UrlType::SoundCloud(SoundCloudType::Set),
            _ => UrlType::SoundCloud(SoundCloudType::Track), // Assuming anything other than "sets" is a track
        };

        let name = parts.next().expect("Found track or set name");
        return (url_type, name.to_string());
    } else {
        eprintln!("ERROR: Invalid host, only open.spotify.com and soundcloud.com are supported");
        std::process::exit(1);
    }
}
