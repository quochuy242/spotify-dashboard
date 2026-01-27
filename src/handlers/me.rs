use axum::{
    extract::State,
    response::IntoResponse,
};
use crate::state::AppState;
use tracing::{error, warn};
use rspotify::clients::OAuthClient;


pub async fn me(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let guard = state.spotify.lock().await;

    let spotify = match guard.as_ref() {
        Some(s) => s,
        None => {
            warn!("User not authenticated");
            return "User not authenticated".into_response();
        }
    };

    match spotify.current_user().await {
        Ok(user) => format!(
            "Logged in as {}, ({})",
            user.display_name.unwrap_or_default(),
            user.email.unwrap_or_default(),
        ).into_response(),
        Err(err) => {
            error!("Spotify API error: {:?}", err);
            "Failed to fetch user".into_response()
        }
    }
}