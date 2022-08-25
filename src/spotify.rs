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
    let oauth = OAuth::from_env(scopes!(
        "user-modify-playback-state",
        "user-read-currently-playing",
        "user-read-playback-state",
        "user-read-private"
    ))
    .unwrap();
    let config = Config {
        token_cached: true,
        token_refreshing: true,
        ..Default::default()
    };

    let spotify = AuthCodeSpotify::with_config(creds, oauth, config);
    let url = spotify.get_authorize_url(false)?;
    spotify.prompt_for_token(&url).await?;

    println!("Connected to Spotify");
    Ok(SpotifyClie