# Spotify Dashboard Telegram Bot

A Telegram bot that lets you interact with Spotify directly. Check your top tracks, artists, recently played songs, and user profile all from Telegram.

## Features

- 🔐 **OAuth2 Authentication** with Spotify
- 🎵 View your top tracks
- 🎤 View your top artists  
- ⏱️ See recently played songs
- 👤 Check your Spotify profile info
- Per-user session management

## Setup

### Prerequisites

- Rust 1.70+ (for building)
- A Telegram bot token (get from [@BotFather](https://t.me/botfather))
- Spotify API credentials (get from [Spotify Developer Dashboard](https://developer.spotify.com/dashboard))

### Installation

1. **Clone the repo**
   ```bash
   git clone <repo-url>
   cd spotify-dashboard
   ```

2. **Setup environment variables**
   ```bash
   cp .env.example .env
   ```
   
   Edit `.env` with:
   - `TELOXIDE_TOKEN`: Your Telegram bot token
   - `RSPOTIFY_CLIENT_ID`: From Spotify Developer Dashboard
   - `RSPOTIFY_CLIENT_SECRET`: From Spotify Developer Dashboard
   - `RSPOTIFY_REDIRECT_URI`: OAuth callback URL (can be anything like http://localhost:3000/callback)

3. **Build & Run**
   ```bash
   cargo build --release
   ./target/release/spotify-dashboard
   ```

## Bot Commands

- `/help` - Show all available commands
- `/login` - Authenticate with Spotify
- `/me` - Show your Spotify profile info
- `/top_tracks` - Show your top 10 tracks
- `/top_artists` - Show your top 10 artists
- `/recently_played` - Show your 10 recently played tracks

## Architecture

The project was migrated from an Axum web server to a Telegram bot using **teloxide**:

- `src/main.rs` - Bot entry point and dispatcher setup
- `src/bot/` - Telegram bot handlers and commands
- `src/handlers/` - Spotify API interaction logic (reused from Axum version)
- `src/auth/` - Spotify OAuth2 authentication
- `src/models/` - Data structures for Spotify API responses
- `src/utils/` - Helper utilities (stream collection, etc)

### Key Dependencies

- **teloxide** - Telegram bot framework
- **rspotify** - Spotify Web API client
- **tokio** - Async runtime
- **serde** - Serialization/deserialization