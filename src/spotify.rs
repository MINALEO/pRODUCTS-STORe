use anyhow::Context;
use rspotify::{
    model::{FullTrack, Market, PlayableItem, SearchResult, SearchType, TrackId},
    prelude::{BaseClient, OAuthClient, PlayableId::Track},
    scopes, AuthCodeS