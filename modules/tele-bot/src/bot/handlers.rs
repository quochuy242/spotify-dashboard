use rspotify::clients::{BaseClient, OAuthClient};
use rspotify::model::Market;
use rspotify::model::SearchResult;
use rspotify::model::SearchType;
use rspotify::AuthCodeSpotify;
use teloxide::prelude::*;
use teloxide::types::InlineKeyboardMarkup;
use tokio::sync::Mutex;
use tracing::error;

use crate::auth::spotify::{spotify_credentials, spotify_oauth};
use crate::state::AppState;
use crate::utils::stream::collect_stream;

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
            let help_text = "<b>🎵 Spotify Dashboard Bot</b>\n\n\
                 <b>Available Commands:</b>\n\n\
                 <code>/login</code> - Authenticate with Spotify\n\
                 <code>/me</code> - View your profile\n\
                 <code>/top_tracks</code> - Your 10 most played tracks\n\
                 <code>/top_artists</code> - Your 10 most played artists\n\
                 <code>/recently_played</code> - Last 10 tracks you played\n\
                 <code>/search query</code> - Search for a track\n\
                 <code>/playlists</code> - List your playlists\n\
                 <code>/playlist name</code> - View playlist details\n\
                 <code>/create_playlist name</code> - Create a new playlist\n\
                 <code>/add_to_playlist song | playlist</code> - Add song to playlist\n\n\
                 <b>Getting Started:</b>\n\
                 Tap <code>/login</code> to connect your Spotify account.";
            bot.send_message(chat_id, help_text)
                .parse_mode(teloxide::types::ParseMode::Html)
                .await?;
        }

        Command::Login => {
            let spotify = AuthCodeSpotify::new(spotify_credentials(), spotify_oauth());
            let url = match spotify.get_authorize_url(false) {
                Ok(u) => u,
                Err(e) => {
                    error!("Failed to get auth URL: {e}");
                    let err_msg = "<b>❌ Authentication Error</b>\n\n\
                                   Failed to generate login URL. Please try again later.";
                    bot.send_message(chat_id, err_msg)
                        .parse_mode(teloxide::types::ParseMode::Html)
                        .await?;
                    return Ok(());
                }
            };

            // Create inline keyboard with login button
            let kb =
                InlineKeyboardMarkup::new(vec![vec![teloxide::types::InlineKeyboardButton::url(
                    "🔐 Login with Spotify".to_string(),
                    url.parse().expect("valid URL"),
                )]]);

            let login_msg = "<b>🎵 Spotify Authentication</b>\n\n\
                             Click the button below to authorize this bot with your Spotify account.\n\n\
                             ✓ We'll never post to your account\n\
                             ✓ Your data stays private";
            bot.send_message(chat_id, login_msg)
                .parse_mode(teloxide::types::ParseMode::Html)
                .reply_markup(kb)
                .await?;
        }

        Command::Me => {
            let state = get_or_create_state(chat_id.0).await;
            match get_me(&state).await {
                Ok(response) => {
                    bot.send_message(chat_id, response)
                        .parse_mode(teloxide::types::ParseMode::Html)
                        .await?;
                }
                Err(e) => {
                    let err_msg = format!("<b>❌ Error</b>\n\n{}", e);
                    bot.send_message(chat_id, err_msg)
                        .parse_mode(teloxide::types::ParseMode::Html)
                        .await?;
                }
            }
        }

        Command::TopTracks => {
            let state = get_or_create_state(chat_id.0).await;
            match get_top_tracks(&state).await {
                Ok(response) => {
                    bot.send_message(chat_id, response)
                        .parse_mode(teloxide::types::ParseMode::Html)
                        .await?;
                }
                Err(e) => {
                    let err_msg = format!("<b>❌ Error</b>\n\n{}", e);
                    bot.send_message(chat_id, err_msg)
                        .parse_mode(teloxide::types::ParseMode::Html)
                        .await?;
                }
            }
        }

        Command::TopArtists => {
            let state = get_or_create_state(chat_id.0).await;
            match get_top_artists(&state).await {
                Ok(response) => {
                    bot.send_message(chat_id, response)
                        .parse_mode(teloxide::types::ParseMode::Html)
                        .await?;
                }
                Err(e) => {
                    let err_msg = format!("<b>❌ Error</b>\n\n{}", e);
                    bot.send_message(chat_id, err_msg)
                        .parse_mode(teloxide::types::ParseMode::Html)
                        .await?;
                }
            }
        }

        Command::RecentlyPlayed => {
            let state = get_or_create_state(chat_id.0).await;
            match get_recently_played(&state).await {
                Ok(response) => {
                    bot.send_message(chat_id, response)
                        .parse_mode(teloxide::types::ParseMode::Html)
                        .await?;
                }
                Err(e) => {
                    let err_msg = format!("<b>❌ Error</b>\n\n{}", e);
                    bot.send_message(chat_id, err_msg)
                        .parse_mode(teloxide::types::ParseMode::Html)
                        .await?;
                }
            }
        }

        Command::Search(query) => {
            let state = get_or_create_state(chat_id.0).await;
            match search_track(&state, &query).await {
                Ok(response) => {
                    bot.send_message(chat_id, response)
                        .parse_mode(teloxide::types::ParseMode::Html)
                        .await?;
                }
                Err(e) => {
                    let err_msg = format!("<b>❌ Error</b>\n\n{}", e);
                    bot.send_message(chat_id, err_msg)
                        .parse_mode(teloxide::types::ParseMode::Html)
                        .await?;
                }
            }
        }

        Command::Playlists => {
            let state = get_or_create_state(chat_id.0).await;
            match list_playlists(&state).await {
                Ok(response) => {
                    bot.send_message(chat_id, response)
                        .parse_mode(teloxide::types::ParseMode::Html)
                        .await?;
                }
                Err(e) => {
                    let err_msg = format!("<b>❌ Error</b>\n\n{}", e);
                    bot.send_message(chat_id, err_msg)
                        .parse_mode(teloxide::types::ParseMode::Html)
                        .await?;
                }
            }
        }

        Command::Playlist(playlist_name) => {
            let state = get_or_create_state(chat_id.0).await;
            match get_playlist(&state, &playlist_name).await {
                Ok(response) => {
                    bot.send_message(chat_id, response)
                        .parse_mode(teloxide::types::ParseMode::Html)
                        .await?;
                }
                Err(e) => {
                    let err_msg = format!("<b>❌ Error</b>\n\n{}", e);
                    bot.send_message(chat_id, err_msg)
                        .parse_mode(teloxide::types::ParseMode::Html)
                        .await?;
                }
            }
        }

        Command::CreatePlaylist(playlist_name) => {
            let state = get_or_create_state(chat_id.0).await;
            match create_playlist(&state, &playlist_name).await {
                Ok(response) => {
                    bot.send_message(chat_id, response)
                        .parse_mode(teloxide::types::ParseMode::Html)
                        .await?;
                }
                Err(e) => {
                    let err_msg = format!("<b>❌ Error</b>\n\n{}", e);
                    bot.send_message(chat_id, err_msg)
                        .parse_mode(teloxide::types::ParseMode::Html)
                        .await?;
                }
            }
        }

        Command::AddToPlaylist(input) => {
            let state = get_or_create_state(chat_id.0).await;
            // Parse input: "song_name | playlist_name"
            let parts: Vec<&str> = input.split('|').collect();
            if parts.len() != 2 {
                let err_msg = "<b>❌ Invalid Format</b>\n\n\
                               Usage: <code>/add_to_playlist song_name | playlist_name</code>";
                bot.send_message(chat_id, err_msg)
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .await?;
                return Ok(());
            }

            let song_name = parts[0].trim();
            let playlist_name = parts[1].trim();

            match add_to_playlist(&state, song_name, playlist_name).await {
                Ok(response) => {
                    bot.send_message(chat_id, response)
                        .parse_mode(teloxide::types::ParseMode::Html)
                        .await?;
                }
                Err(e) => {
                    let err_msg = format!("<b>❌ Error</b>\n\n{}", e);
                    bot.send_message(chat_id, err_msg)
                        .parse_mode(teloxide::types::ParseMode::Html)
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
    let spotify = guard
        .as_ref()
        .ok_or_else(|| "Please authenticate first using <code>/login</code>".to_string())?;

    match spotify.current_user().await {
        Ok(user) => {
            let display_name = user.display_name.unwrap_or_else(|| "User".to_string());
            let email = user.email.unwrap_or_else(|| "No email".to_string());

            let profile = format!(
                "<b>👤 Your Spotify Profile</b>\n\n\
                 <b>Name:</b> {}\n\
                 <b>Email:</b> <code>{}</code>\n\
                 <b>Status:</b> ✅ Connected",
                html_escape(&display_name),
                html_escape(&email)
            );
            Ok(profile)
        }
        Err(err) => {
            error!("Spotify API error: {:?}", err);
            Err("Failed to fetch profile. Please try again.".to_string())
        }
    }
}

async fn get_top_tracks(state: &AppState) -> Result<String, String> {
    let guard = state.spotify.lock().await;
    let spotify = guard
        .as_ref()
        .ok_or_else(|| "Please authenticate first using <code>/login</code>".to_string())?;

    let stream = spotify.current_user_top_tracks(None);
    let tracks = collect_stream(stream, |track| crate::models::spotify::Track {
        name: track.name,
        artists: track.artists.into_iter().map(|a| a.name).collect(),
    })
    .await
    .map_err(|_| "Failed to fetch top tracks. Please try again.".to_string())?;

    if tracks.is_empty() {
        return Ok("📭 No top tracks found. Start listening to see your favorites!".to_string());
    }

    let mut response = "<b>🎵 Your Top Tracks</b>\n\n".to_string();
    for (idx, track) in tracks.iter().enumerate().take(10) {
        let artists = track.artists.join(", ");
        response.push_str(&format!(
            "<b>{}</b>. {}\n<i>{}</i>\n\n",
            idx + 1,
            html_escape(&track.name),
            html_escape(&artists)
        ));
    }

    Ok(response)
}

async fn get_top_artists(state: &AppState) -> Result<String, String> {
    let guard = state.spotify.lock().await;
    let spotify = guard
        .as_ref()
        .ok_or_else(|| "Please authenticate first using <code>/login</code>".to_string())?;

    let stream = spotify.current_user_top_artists(None);
    let artists = collect_stream(stream, |artist| crate::models::spotify::Artist {
        name: artist.name,
        genres: artist.genres,
    })
    .await
    .map_err(|_| "Failed to fetch top artists. Please try again.".to_string())?;

    if artists.is_empty() {
        return Ok(
            "📭 No top artists found. Start following artists to see your favorites!".to_string(),
        );
    }

    let mut response = "<b>🎤 Your Top Artists</b>\n\n".to_string();
    for (idx, artist) in artists.iter().enumerate().take(10) {
        let genres = if !artist.genres.is_empty() {
            format!("\n<i>{}</i>", html_escape(&artist.genres.join(", ")))
        } else {
            String::new()
        };
        response.push_str(&format!(
            "<b>{}</b>. {}{}\n\n",
            idx + 1,
            html_escape(&artist.name),
            genres
        ));
    }

    Ok(response)
}

async fn get_recently_played(state: &AppState) -> Result<String, String> {
    let guard = state.spotify.lock().await;
    let spotify = guard
        .as_ref()
        .ok_or_else(|| "Please authenticate first using <code>/login</code>".to_string())?;

    let result = spotify
        .current_user_recently_played(None, None)
        .await
        .map_err(|_| "Failed to fetch recent tracks. Please try again.".to_string())?;

    if result.items.is_empty() {
        return Ok("📭 No recently played tracks found.".to_string());
    }

    let mut response = "<b>⏱️ Recently Played</b>\n\n".to_string();
    for (idx, item) in result.items.iter().enumerate().take(10) {
        let track = &item.track;
        let artists: Vec<String> = track.artists.iter().map(|a| a.name.clone()).collect();
        response.push_str(&format!(
            "<b>{}</b>. {}\n<i>{}</i>\n\n",
            idx + 1,
            html_escape(&track.name),
            html_escape(&artists.join(", "))
        ));
    }

    Ok(response)
}

async fn search_track(state: &AppState, query: &str) -> Result<String, String> {
    let guard = state.spotify.lock().await;
    let spotify = guard
        .as_ref()
        .ok_or_else(|| "Please authenticate first using <code>/login</code>".to_string())?;

    let query = query.trim();
    if query.is_empty() {
        return Err("Please provide a search query.".to_string());
    }

    // Search in whole Spotify database
    let result = spotify
        .search(
            query,
            SearchType::Track,
            Some(Market::FromToken),
            None,
            Some(5),
            None,
        )
        .await
        .map_err(|_| "Failed to search tracks. Please try again.".to_string())?;

    let page = match result {
        SearchResult::Tracks(page) => page,
        _ => return Err("Failed to search tracks. Please try again.".to_string()),
    };

    if page.items.is_empty() {
        return Ok(format!(
            "📭 <b>Search Results for \"{}\"</b>\n\nNo tracks found.",
            html_escape(query)
        ));
    }

    let mut response = format!(
        "<b>📭 Search Results for \"{}\"</b>\n\n",
        html_escape(query)
    );
    for (idx, track) in page.items.iter().enumerate() {
        let artists: Vec<String> = track
            .artists
            .iter()
            .map(|a| a.name.clone())
            .collect::<Vec<_>>();
        response.push_str(&format!(
            "<b>{}</b>. {}\n<i>{}</i>\n\n",
            idx + 1,
            html_escape(&track.name),
            html_escape(&artists.join(", "))
        ));
    }

    Ok(response)
}

async fn list_playlists(state: &AppState) -> Result<String, String> {
    let guard = state.spotify.lock().await;
    let spotify = guard
        .as_ref()
        .ok_or_else(|| "Please authenticate first using <code>/login</code>".to_string())?;

    let stream = spotify.current_user_playlists();
    let playlists = collect_stream(stream, |p| p)
        .await
        .map_err(|_| "Failed to fetch playlists. Please try again.".to_string())?;

    if playlists.is_empty() {
        return Ok("📭 <b>Your Playlists</b>\n\nNo playlists found. Create one with <code>/create_playlist</code>".to_string());
    }

    let mut response = "<b>📋 Your Playlists</b>\n\n".to_string();
    for (idx, playlist) in playlists.iter().enumerate().take(20) {
        let track_count = playlist.tracks.total;
        response.push_str(&format!(
            "<b>{}</b>. {}\n<i>{} tracks</i>\n\n",
            idx + 1,
            html_escape(&playlist.name),
            track_count
        ));
    }

    Ok(response)
}

async fn get_playlist(state: &AppState, playlist_name: &str) -> Result<String, String> {
    let guard = state.spotify.lock().await;
    let spotify = guard
        .as_ref()
        .ok_or_else(|| "Please authenticate first using <code>/login</code>".to_string())?;

    if playlist_name.is_empty() {
        return Err("Please provide a playlist name.".to_string());
    }

    // Get user playlists
    let stream = spotify.current_user_playlists();
    let playlists = collect_stream(stream, |p| p)
        .await
        .map_err(|_| "Failed to fetch playlists. Please try again.".to_string())?;

    // Find matching playlist
    let playlist = playlists
        .iter()
        .find(|p| p.name.to_lowercase() == playlist_name.to_lowercase())
        .ok_or_else(|| format!("Playlist \"{}\" not found.", html_escape(playlist_name)))?;

    let response = format!(
        "<b>📋 {}</b>\n\n<b>Tracks:</b> {}\n\n",
        html_escape(&playlist.name),
        playlist.tracks.total
    );

    // Note: Showing playlist details without full track list due to API complexity
    // Use /playlist_name command to view details

    Ok(response)
}

async fn create_playlist(state: &AppState, playlist_name: &str) -> Result<String, String> {
    let guard = state.spotify.lock().await;
    let spotify = guard
        .as_ref()
        .ok_or_else(|| "Please authenticate first using <code>/login</code>".to_string())?;

    if playlist_name.is_empty() {
        return Err("Please provide a playlist name.".to_string());
    }

    let user = spotify
        .current_user()
        .await
        .map_err(|_| "Failed to fetch user info.".to_string())?;

    spotify
        .user_playlist_create(
            user.id,
            playlist_name,
            Some(false),
            Some(false),
            Some("Created with Spotify Dashboard Bot"),
        )
        .await
        .map_err(|_| "Failed to create playlist. Please try again.".to_string())?;

    Ok(format!(
        "✅ <b>Playlist Created</b>\n\n\
         <b>Name:</b> {}\n\n\
         You can now add tracks to this playlist.",
        html_escape(playlist_name)
    ))
}

async fn add_to_playlist(
    state: &AppState,
    song_name: &str,
    playlist_name: &str,
) -> Result<String, String> {
    let guard = state.spotify.lock().await;
    let spotify = guard
        .as_ref()
        .ok_or_else(|| "Please authenticate first using <code>/login</code>".to_string())?;

    if song_name.is_empty() || playlist_name.is_empty() {
        return Err("Please provide both song name and playlist name.".to_string());
    }

    // Search in user's saved tracks
    let stream = spotify.current_user_saved_tracks(Some(Market::FromToken));
    let saved_tracks = collect_stream(stream, |item| item.track)
        .await
        .map_err(|_| "Failed to fetch your saved tracks.".to_string())?;

    let query_lower = song_name.to_lowercase();
    let track = saved_tracks
        .iter()
        .find(|t| t.name.to_lowercase().contains(&query_lower))
        .ok_or_else(|| {
            format!(
                "Track \"{}\" not found in your library. Please save it first.",
                html_escape(song_name)
            )
        })?;

    // Get user playlists
    let stream = spotify.current_user_playlists();
    let playlists = collect_stream(stream, |p| p)
        .await
        .map_err(|_| "Failed to fetch playlists.".to_string())?;

    // Find matching playlist
    let playlist = playlists
        .iter()
        .find(|p| p.name.to_lowercase() == playlist_name.to_lowercase())
        .ok_or_else(|| format!("Playlist \"{}\" not found.", html_escape(playlist_name)))?;

    // Add track to playlist
    use rspotify::model::PlayableId;
    if let Some(track_id) = &track.id {
        let playable_id = PlayableId::Track(track_id.clone());
        spotify
            .playlist_add_items(playlist.id.clone(), vec![playable_id], None)
            .await
            .map_err(|_| "Failed to add track to playlist.".to_string())?;
    } else {
        return Err("Track ID not available.".to_string());
    }

    Ok(format!(
        "✅ <b>Track Added</b>\n\n\
         <b>Song:</b> {}\n\
         <b>Playlist:</b> {}\n\n\
         Track successfully added to your playlist!",
        html_escape(&track.name),
        html_escape(&playlist.name)
    ))
}

// Helper function to escape HTML special characters
fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
