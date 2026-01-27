use crate::{
    error::ApiError,
    models::spotify::{ApiResponse, Artist, Track},
    state::{AppState, require_spotify},
    utils::stream::collect_stream,
};
use axum::{
    extract::State,
    response::{IntoResponse, Json},
};
use rspotify::{clients::OAuthClient, prelude::Id};

pub async fn top_tracks(State(state): State<AppState>) -> Result<impl IntoResponse, ApiError> {
    let spotify = require_spotify(&state).await?;

    // STREAM endpoint
    let stream = spotify.current_user_top_tracks(None);

    let tracks = collect_stream(stream, |track| Track {
        id: track.id.unwrap().id().to_string(),
        name: track.name,
        artists: track.artists.into_iter().map(|a| a.name).collect(),
        preview_url: track.preview_url,
    })
    .await
    .map_err(|_| ApiError::Spotify)?;

    Ok(Json(ApiResponse { data: tracks }))
}

pub async fn top_artists(State(state): State<AppState>) -> Result<impl IntoResponse, ApiError> {

    let spotify = require_spotify(&state).await?;

    // STREAM endpoint
    let stream = spotify.current_user_top_artists(None);
    let artists = collect_stream(stream, |artist| Artist {
        id: artist.id.id().to_string(),
        name: artist.name,
        genres: artist.genres,
    })
    .await
    .map_err(|_| ApiError::Spotify)?;

    Ok(Json(ApiResponse { data: artists }))
}
