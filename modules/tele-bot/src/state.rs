use rspotify::AuthCodeSpotify;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
    pub spotify: Arc<Mutex<Option<AuthCodeSpotify>>>,
}
