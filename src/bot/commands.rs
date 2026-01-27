use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "Spotify Dashboard Bot Commands"
)]
pub enum Command {
    #[command(description = "show help")]
    Help,

    #[command(description = "authenticate with Spotify")]
    Login,

    #[command(description = "show current user info")]
    Me,

    #[command(description = "show top tracks")]
    TopTracks,

    #[command(description = "show top artists")]
    TopArtists,

    #[command(description = "show recently played")]
    RecentlyPlayed,

    #[command(description = "search for a track (usage: /search song_name)")]
    Search(String),

    #[command(description = "list your playlists")]
    Playlists,

    #[command(description = "show playlist details (usage: /playlist playlist_name)")]
    Playlist(String),

    #[command(description = "create a new playlist (usage: /create_playlist playlist_name)")]
    CreatePlaylist(String),

    #[command(description = "add track to playlist (usage: /add_to_playlist song_name | playlist_name)")]
    AddToPlaylist(String),
}
