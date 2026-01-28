#[derive(Clone)]
pub struct Track {
    pub name: String,
    pub artists: Vec<String>,
}

#[derive(Clone)]
pub struct Artist {
    pub name: String,
    pub genres: Vec<String>,
}
