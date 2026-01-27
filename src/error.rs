use axum::{http::StatusCode, response::IntoResponse};

pub enum ApiError {
    Unauthorized,
    Spotify,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ApiError::Unauthorized =>
                (StatusCode::UNAUTHORIZED, "Not authenticated").into_response(),
            ApiError::Spotify =>
                (StatusCode::INTERNAL_SERVER_ERROR, "Spotify API error").into_response(),
        }
    }
}
