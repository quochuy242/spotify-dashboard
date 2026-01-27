use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
};
use std::collections::HashMap;
use tracing::{error, info};

use rspotify::{AuthCodeSpotify, clients::OAuthClient};

use crate::auth::spotify::{spotify_credentials, spotify_oauth};
use crate::state::AppState;

pub async fn login() -> impl IntoResponse {
    let spotify = AuthCodeSpotify::new(spotify_credentials(), spotify_oauth());
    let url = spotify.get_authorize_url(false).unwrap();
    Redirect::temporary(&url)
}

pub async fn callback(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let code = match params.get("code") {
        Some(code) => code,
        None => return "No code received".into_response(),
    };

    let spotify = AuthCodeSpotify::new(spotify_credentials(), spotify_oauth());

    if let Err(err) = spotify.request_token(code).await {
        error!("Token exchange failed: {err}");
        return "Token exchange failed".into_response();
    }

    info!("Spotify token acquired");
    *state.spotify.lock().await = Some(spotify);
    Redirect::to("/api/me").into_response()
}
