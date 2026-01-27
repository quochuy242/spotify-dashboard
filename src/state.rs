use std::sync::Arc;
use rspotify::AuthCodeSpotify;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
    pub spotify: Arc<Mutex<Option<AuthCodeSpotify>>>,
}