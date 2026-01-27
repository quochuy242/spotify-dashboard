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
}
