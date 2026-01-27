use serde::Serialize;

#[derive(Serialize)]
pub struct Track {
    pub id: String,
    pub name: String, 
    pub artists: Vec<String>,
    pub preview_url: Option<String>,
}

#[derive(Serialize)]
pub struct Artist {
    pub id: String,
    pub name: String,
    pub genres: Vec<String>,
}

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub data: T,
}