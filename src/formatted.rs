use anyhow::Error;
use matrix_sdk::ruma::events::room::message::RoomMessageEventContent;
use rspotify::{model::FullTrack, ClientError};

pub fn track(track: &FullTrack