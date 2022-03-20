
use std::env;

use dotenv::dotenv;
use lazy_static::lazy_static;
use matrix_sdk::{
    config::SyncSettings,
    event_handler::Ctx,
    room::Room,
    ruma::{