
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