use std::sync::Arc;

use rspotify::{AuthCodeSpotify};
use tokio::sync::Mutex;
use crate::error::ApiError;

#[derive(Clone)]
pub struct AppState {
    pub spotify: Arc<Mutex<Option<AuthCodeSpotify>>>,
}

pub async fn require_spotify(
    state: &AppState,
) -> Result<std::sync::Arc<AuthCodeSpotify>, ApiError> {
    let guard = state.spotify.lock().await;
    match guard.as_ref() {
        Some(spotify) => Ok(spotify.clone().into()),
        None => Err(ApiError::Unauthorized),
    }
}