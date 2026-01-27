use rspotify::{Credentials, OAuth};

pub fn spotify_oauth() -> OAuth {
    OAuth {
        redirect_uri: std::env::var("SPOTIFY_REDIRECT_URI")
            .expect("SPOTIFY_REDIRECT_URI not set"),
        scopes: rspotify::scopes!(
            "user-top-read",
            "user-read-recently-played"
        ),
        ..Default::default()
    }
}

pub fn spotify_credentials() -> Credentials {
    Credentials::new(
        &std::env::var("SPOTIFY_CLIENT_ID").expect("SPOTIFY_CLIENT_ID not set"),
        &std::env::var("SPOTIFY_CLIENT_SECRET").expect("SPOTIFY_CLIENT_SECRET not set"),
    )
}