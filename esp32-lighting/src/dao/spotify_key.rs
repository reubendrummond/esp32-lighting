use common::spotify::SpotifyKey;

pub trait SpotifyKeyDao {
    fn get_spotify_key(&self) -> Option<&SpotifyKey>;
    fn save_spotify_key(&mut self, key: SpotifyKey) -> Result<(), anyhow::Error>;
    fn clear(&mut self) -> Result<(), anyhow::Error>;
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

    fn clear(&mut self) -> Result<(), anyhow::Error> {
        self.key = None;
        Ok(())
    }
}
