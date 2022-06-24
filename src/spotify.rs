use anyhow::Context;
use rspotify::{
    model::{FullTrack, Market, PlayableItem, SearchResult, SearchType, TrackId},
    prelude::{BaseClient, OAuthClient, PlayableId::Track},
    scopes, AuthCodeSpotify, Config, Credentials, OAuth,
};

#[derive(Clone)]
pub struct SpotifyClient {
    client: AuthCodeSpotify,
}

pub async fn login() -> anyhow::Result<SpotifyClient> {
    let creds = Credentials::from_env().unwrap();
    let oa