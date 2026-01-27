use crate::{
    error::ApiError,
    models::spotify::{ApiResponse, Track},
    state::{AppState, require_spotify},
};
use axum::{
    extract::State,
    response::{IntoResponse, Json},
};
use rspotify::{clients::OAuthClient, prelude::Id};

// FUTURE endpoint (cursor-based)
pub async fn recently_played(State(state): State<AppState>) -> Result<impl IntoResponse, ApiError> {

    let spotify = require_spotify(&state).await?;

    // FUTURE endpoint
    let result = spotify
        .current_user_recently_played(None, None)
        .await
        .map_err(|_| ApiError::Spotify)?;
    let tracks: Vec<Track> = result
        .items
        .into_iter()
        .map(|item| {
            let track = item.track;
            Track {
                id: track.id.unwrap().id().to_string(),
                name: track.name,
                artists: track
                    .artists
                    .into_iter()
                    .map(|artist| artist.name)
                    .collect(),
                preview_url: track.preview_url,
            }
        })
        .collect();

    Ok(Json(ApiResponse { data: tracks }))
}
