use rayon::prelude::*;
use std::path::{Path, PathBuf};

use crate::{ffmpeg, spotify, youtube};
use ffmpeg::SpotifyMetadata;
use spotify::access_token::AccessToken;
use spotify::api::{fetch_album, fetch_playlist, fetch_track};
use youtube::api::search_videos;
use youtube::download::download_video;

pub fn download_playlist(
    access_token: &AccessToken,
    id: &str,
    yt_dlp_path: &Path,
    ffmpeg_path: &PathBuf,
) {
    let playlist = fetch_playlist(access_token.get_token(), id);

    println!("Downloading playlist: {}", playlist.name);

    playlist.tracks.items.par_iter().for_each(|playlist_track| {
        let track = &playlist_track.track;

        let qry = format!(
            "{} - {}",
            track.name,
            track
                .artists
                .iter()
                .map(|artist| artist.name.clone())
                .collect::<Vec<String>>()
                .join(", ")
        );

        let videos = search_videos(&qry);

        // Take the video in the first 5 results with the most similar duration as the track
        let video = videos
            .iter()
            .take(5)
            .min_by_key(|&video| video.duration_ms.wrapping_sub(track.duration_ms))
            .unwrap();

        let input_file = download_video(yt_dlp_path, video, Some(Path::new(&playlist.name)))
            .map_err(|err| {
                eprintln!("ERROR: failed to download video: {err}");
            })
            .unwrap();

        let metadata = SpotifyMetadata {
            title: track.name.clone(),
            artists: track.artists.clone(),
            album_artists: track.album.artists.clone(),
            album_name: track.album.name.clone(),
            total_tracks: track.album.total_tracks,
            track_number: track.track_number,
            release_date: track.album.release_date.clone(),
            album_cover_url: track.album.images[0].url.clone(),
        };

        ffmpeg::metadata_and_to_mp3(ffmpeg_path, &input_file, &metadata);
    });
}

pub fn download_album(
    access_token: &AccessToken,
    id: &str,
    yt_dlp_path: &Path,
    ffmpeg_path: &PathBuf,
) {
    let album = fetch_album(access_token.get_token(), id);

    println!("Downloading album: {}", album.name);

    album.tracks.items.par_iter().for_each(|track| {
        let qry = format!(
            "{} - {}",
            track.name,
            track
                .artists
                .iter()
                .map(|artist| artist.name.clone())
                .collect::<Vec<String>>()
                .join(", ")
        );

        let videos = search_videos(&qry);

        // Take the video in the first 5 results with the most similar duration as the track
        let video = videos
            .iter()
            .take(5)
            .min_by_key(|&video| video.duration_ms.wrapping_sub(track.duration_ms))
            .unwrap();

        let input_file = download_video(yt_dlp_path, video, Some(Path::new(&album.name)))
            .map_err(|err| {
                eprintln!("ERROR: failed to download video: {err}");
            })
            .unwrap();

        let metadata = SpotifyMetadata {
            title: track.name.clone(),
            artists: track.artists.clone(),
            album_artists: album.artists.clone(),
            album_name: album.name.clone(),
            total_tracks: album.tracks.total,
            track_number: track.track_number,
            release_date: album.release_date.clone(),
            album_cover_url: album.images[0].url.clone(),
        };

        ffmpeg::metadata_and_to_mp3(ffmpeg_path, &input_file, &metadata);
    });
}

pub fn download_track(
    access_token: &AccessToken,
    id: &str,
    yt_dlp_path: &Path,
    ffmpeg_path: &PathBuf,
) {
    let track = fetch_track(access_token.get_token(), id);

    println!("Downloading track: {}", track.name);

    let qry = format!(
        "{} - {}",
        track.name,
        track
            .artists
            .iter()
            .map(|artist| artist.name.clone())
            .collect::<Vec<String>>()
            .join(", ")
    );

    let videos = search_videos(&qry);

    // Take the video in the first 5 results with the most similar duration as the track
    let video = videos
        .iter()
        .take(5)
        .min_by_key(|&video| video.duration_ms.wrapping_sub(track.duration_ms))
        .unwrap();

    let input_file = download_video(yt_dlp_path, video, None)
        .map_err(|err| {
            eprintln!("ERROR: failed to download video: {err}");
        })
        .unwrap();

    let metadata = SpotifyMetadata {
        title: track.name,
        artists: track.artists,
        album_artists: track.album.artists,
        album_name: track.album.name,
        total_tracks: track.album.total_tracks,
        track_number: track.track_number,
        release_date: track.album.release_date,
        album_cover_url: track.album.images[0].url.clone(),
    };

    ffmpeg::metadata_and_to_mp3(ffmpeg_path, &input_file, &metadata);
}
