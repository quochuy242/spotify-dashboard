use teloxide::prelude::*;
use teloxide::types::InlineKeyboardMarkup;
use teloxide::utils::command::BotCommands;
use tokio::sync::Mutex;

use crate::auth::spotify::{spotify_credentials, spotify_oauth};
use crate::state::AppState;
use crate::utils::stream::collect_stream;
use rspotify::{AuthCodeSpotify, clients::OAuthClient};
use tracing::error;

use super::commands::Command;

// Global state for storing user Spotify sessions per chat
lazy_static::lazy_static! {
    static ref CHAT_STATES: Mutex<std::collections::HashMap<i64, AppState>> = 
        Mutex::new(std::collections::HashMap::new());
}

pub fn schema() -> teloxide::dispatching::UpdateHandler<teloxide::RequestError> {
    Update::filter_message()
        .filter_command::<Command>()
        .endpoint(handle_commands)
}

async fn handle_commands(
    bot: Bot,
    msg: Message,
    cmd: Command,
) -> Result<(), teloxide::RequestError> {
    let chat_id = msg.chat.id;

    match cmd {
        Command::Help => {
            bot.send_message(chat_id, Command::descriptions().to_string())
                .await?;
        }

        Command::Login => {
            let spotify = AuthCodeSpotify::new(spotify_credentials(), spotify_oauth());
            let url = match spotify.get_authorize_url(false) {
                Ok(u) => u,
                Err(e) => {
                    error!("Failed to get auth URL: {e}");
                    bot.send_message(chat_id, "❌  Failed to get auth URL")
                        .await?;
                    return Ok(());
                }
            };

            // Create inline keyboard with login button
            let kb = InlineKeyboardMarkup::new(vec![vec![
                teloxide::types::InlineKeyboardButton::url(
                    "🔐 Login with Spotify".to_string(),
                    url.parse().expect("valid URL"),
                ),
            ]]);

            bot.send_message(
                chat_id,
                "Click the button to authenticate with Spotify",
            )
            .reply_markup(kb)
            .await?;
        }

        Command::Me => {
            let state = get_or_create_state(chat_id.0).await;
            match get_me(&state).await {
                Ok(response) => {
                    bot.send_message(chat_id, response).await?;
                }
                Err(e) => {
                    bot.send_message(chat_id, format!("❌  Error: {}", e))
                        .await?;
                }
            }
        }

        Command::TopTracks => {
            let state = get_or_create_state(chat_id.0).await;
            match get_top_tracks(&state).await {
                Ok(response) => {
                    bot.send_message(chat_id, response).await?;
                }
                Err(e) => {
                    bot.send_message(chat_id, format!("❌  Error: {}", e))
                        .await?;
                }
            }
        }

        Command::TopArtists => {
            let state = get_or_create_state(chat_id.0).await;
            match get_top_artists(&state).await {
                Ok(response) => {
                    bot.send_message(chat_id, response).await?;
                }
                Err(e) => {
                    bot.send_message(chat_id, format!("❌  Error: {}", e))
                        .await?;
                }
            }
        }

        Command::RecentlyPlayed => {
            let state = get_or_create_state(chat_id.0).await;
            match get_recently_played(&state).await {
                Ok(response) => {
                    bot.send_message(chat_id, response).await?;
                }
                Err(e) => {
                    bot.send_message(chat_id, format!("❌  Error: {}", e))
                        .await?;
                }
            }
        }
    }

    Ok(())
}

async fn get_or_create_state(chat_id: i64) -> AppState {
    let mut states = CHAT_STATES.lock().await;
    states
        .entry(chat_id)
        .or_insert_with(|| AppState {
            spotify: std::sync::Arc::new(tokio::sync::Mutex::new(None)),
        })
        .clone()
}

async fn get_me(state: &AppState) -> Result<String, String> {
    let guard = state.spotify.lock().await;
    let spotify = guard.as_ref().ok_or_else(|| {
        "Not authenticated. Use /login first.".to_string()
    })?;

    match spotify.current_user().await {
        Ok(user) => {
            let display_name = user.display_name.unwrap_or_default();
            let email = user.email.unwrap_or_default();
            Ok(format!("👤 Logged in as: {}\n📧 Email: {}", display_name, email))
        }
        Err(err) => {
            error!("Spotify API error: {:?}", err);
            Err("Failed to fetch user info".to_string())
        }
    }
}

async fn get_top_tracks(state: &AppState) -> Result<String, String> {
    let guard = state.spotify.lock().await;
    let spotify = guard.as_ref().ok_or_else(|| {
        "Not authenticated. Use /login first.".to_string()
    })?;

    let stream = spotify.current_user_top_tracks(None);
    let tracks = collect_stream(stream, |track| crate::models::spotify::Track {
        name: track.name,
        artists: track.artists.into_iter().map(|a| a.name).collect(),
    })
    .await
    .map_err(|_| "Failed to fetch top tracks".to_string())?;

    if tracks.is_empty() {
        return Ok("No top tracks found.".to_string());
    }

    let mut response = "🎵 **Your Top Tracks:**\n\n".to_string();
    for (idx, track) in tracks.iter().enumerate().take(10) {
        let artists = track.artists.join(", ");
        response.push_str(&format!("{}. {} - {}\n", idx + 1, track.name, artists));
    }

    Ok(response)
}

async fn get_top_artists(state: &AppState) -> Result<String, String> {
    let guard = state.spotify.lock().await;
    let spotify = guard.as_ref().ok_or_else(|| {
        "Not authenticated. Use /login first.".to_string()
    })?;

    let stream = spotify.current_user_top_artists(None);
    let artists = collect_stream(stream, |artist| crate::models::spotify::Artist {
        name: artist.name,
        genres: artist.genres,
    })
    .await
    .map_err(|_| "Failed to fetch top artists".to_string())?;

    if artists.is_empty() {
        return Ok("No top artists found.".to_string());
    }

    let mut response = "🎤 **Your Top Artists:**\n\n".to_string();
    for (idx, artist) in artists.iter().enumerate().take(10) {
        let genres = if !artist.genres.is_empty() {
            format!(" ({})", artist.genres.join(", "))
        } else {
            String::new()
        };
        response.push_str(&format!("{}. {}{}\n", idx + 1, artist.name, genres));
    }

    Ok(response)
}

async fn get_recently_played(state: &AppState) -> Result<String, String> {
    let guard = state.spotify.lock().await;
    let spotify = guard.as_ref().ok_or_else(|| {
        "Not authenticated. Use /login first.".to_string()
    })?;

    let result = spotify
        .current_user_recently_played(None, None)
        .await
        .map_err(|_| "Failed to fetch recently played".to_string())?;

    if result.items.is_empty() {
        return Ok("No recently played tracks found.".to_string());
    }

    let mut response = "⏱️ **Recently Played:**\n\n".to_string();
    for (idx, item) in result.items.iter().enumerate().take(10) {
        let track = &item.track;
        let artists: Vec<String> = track.artists.iter().map(|a| a.name.clone()).collect();
        response.push_str(&format!(
            "{}. {} - {}\n",
            idx + 1,
            track.name,
            artists.join(", ")
        ));
    }

    Ok(response)
}
