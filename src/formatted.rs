use anyhow::Error;
use matrix_sdk::ruma::events::room::message::RoomMessageEventContent;
use rspotify::{model::FullTrack, ClientError};

pub fn track(track: &FullTrack) -> String {
    format!(
        "{} - {}",
        track
            .artists
            .iter()
            .map(|artist| &artist.name)
            .fold(String::new(), |a, b| a + &b + ", ")
            .trim_end_matches(", "),
        track.name
    )
}

pub async fn error(error: Error) -> RoomMessageEventContent {
    match error.downcast::<ClientError>() {
        Ok(ClientError::Http(http)) => match *http {
            rspotify::http::HttpError::StatusCode(error) => {
                RoomMessageEventContent::text_markdown(format!(
                    "```\n{}\n{}\n```",
                    error.stat