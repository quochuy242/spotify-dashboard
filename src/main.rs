mod auth;
mod state;
mod models;
mod utils;
mod error;
mod bot;

use dotenvy::dotenv;
use tracing::info;
use tracing_subscriber::EnvFilter;
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "spotify_dashboard=info,teloxide=info".into()),
        )
        .init();

    let bot = Bot::from_env();
    info!("Spotify Dashboard Telegram Bot started");

    Dispatcher::builder(
        bot,
        bot::handlers::schema(),
    )
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}
