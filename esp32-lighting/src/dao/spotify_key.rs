pub struct SpotifyKey {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: u32,
}

pub trait SpotifyKeyDao {
    fn get_spotify_key(&self) -> Option<&SpotifyKey>;
    fn save_spotify_key(&mut self, key: SpotifyKey) -> Result<(), anyhow::Error>;
}

pub struct SpotifyKeyDaoImpl {
    key: Option<SpotifyKey>,
}

impl SpotifyKeyDaoImpl {
    pub fn new() -> Self {
        Self { key: None }
    }
}

impl SpotifyKeyDao for SpotifyKeyDaoImpl {
    fn get_spotify_key(&self) -> Option<&SpotifyKey> {
        self.key.as_ref()
    }

    fn save_spotify_key(&mut self, key: SpotifyKey) -> Result<(), anyhow::Error> {
        self.key = Some(key);
        Ok(())
    }
}
