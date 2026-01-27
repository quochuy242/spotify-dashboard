mod handlers;
mod auth;
mod state;
mod models;
mod utils;
mod error;

use axum::{routing::get, Router};
use dotenvy::dotenv;
use std::net::SocketAddr;
use tracing::info;
use tracing_subscriber::EnvFilter;
use crate::handlers::{
    auth as handlers_auth,
    me::me,
    top::{top_artists, top_tracks},
    recent::recently_played,
};


#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "spotify_dashboard=info,axum=info".into()),
        )
        .init();

    let state = state::AppState {
        spotify: std::sync::Arc::new(tokio::sync::Mutex::new(None)),
    };

    let app = Router::new()
        .route("/", get(root))
        .route("/auth/login", get(handlers_auth::login))
        .route("/auth/callback", get(handlers_auth::callback))
        .route("/api/me", get(me))
        .route("/api/top-tracks", get(top_tracks))
        .route("/api/top-artists", get(top_artists))
        .route("/api/recently-played", get(recently_played))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Server running at http://{}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app,
    )
    .await
    .unwrap();
}

async fn root() -> &'static str {
    "Spotify Dashboard Backend is running"
}
