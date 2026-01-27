use rspotify::Token;
use std::collections::HashMap;
use tokio::sync::RwLock;

#[derive(Debug)]
pub struct UserAuth {
    pub telegram_id: i64,
    pub token: Token
}

pub type TokenStore = RwLock<HashMap<i64, Token>>;