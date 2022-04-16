
use std::env;

use dotenv::dotenv;
use lazy_static::lazy_static;
use matrix_sdk::{
    config::SyncSettings,
    event_handler::Ctx,
    room::Room,
    ruma::{
        events::room::message::{
            MessageType, OriginalSyncRoomMessageEvent, RoomMessageEventContent,
        },
        RoomId,
    },
    Client,
};
use regex::Regex;
use rspotify::model::{FullTrack, TrackId};
use spotify::SpotifyClient;

mod formatted;
mod spotify;

lazy_static! {
    static ref RX_TRACK_URL: Regex =
        Regex::new(r"https://open.spotify.com/track/([^\?]+)").unwrap();
}

async fn get_queue_handler(spotify: &SpotifyClient) -> anyhow::Result<RoomMessageEventContent> {
    let tracks = spotify
        .get_queue()
        .await?
        .iter()
        .map(|x| formatted::track(x))
        .fold(String::new(), |a, b| a + &b + "\n");

    Ok(RoomMessageEventContent::text_markdown(format!(
        "```\n{}\n```",
        tracks.trim_end()
    )))
}

async fn get_track_handler(
    spotify: &SpotifyClient,
    id: &str,
) -> anyhow::Result<RoomMessageEventContent> {
    let track = spotify
        .get_track(TrackId::from_id_or_uri(id).unwrap())
        .await?;

    queue_track(spotify, &track).await
}

async fn search_track_handler(
    spotify: &SpotifyClient,
    search_term: &str,
) -> anyhow::Result<RoomMessageEventContent> {
    match spotify.search_track(search_term).await? {
        Some(track) => queue_track(spotify, &track).await,
        None => Ok(RoomMessageEventContent::text_plain(format!(
            "No tracks found matching: \"{}\"",
            search_term
        ))),
    }
}

async fn queue_track(
    spotify: &SpotifyClient,
    track: &FullTrack,
) -> anyhow::Result<RoomMessageEventContent> {
    spotify.queue_track(track).await?;

    Ok(RoomMessageEventContent::text_plain(format!(
        "Queued: {}",
        formatted::track(track)
    )))
}

async fn on_room_message(
    event: OriginalSyncRoomMessageEvent,
    room: Room,
    spotify: Ctx<SpotifyClient>,
) {
    if let Room::Joined(room) = room {
        let MessageType::Text(message) = event.content.msgtype else {
            return;
        };
