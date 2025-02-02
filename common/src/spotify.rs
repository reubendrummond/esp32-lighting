pub struct SpotifyKey {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct CurrentSong {
    pub album_url: String,
}
