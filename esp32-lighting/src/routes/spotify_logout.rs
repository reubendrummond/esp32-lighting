use std::sync::{Arc, Mutex};

use esp_idf_svc::http::server;

use crate::dao::spotify_key::SpotifyKeyDao;

pub fn spotify_logout_handler<TSpotifyKeysDao>(
    request: server::Request<&mut server::EspHttpConnection>,
    redirect_uri: &str,
    spotify_keys_dao: Arc<Mutex<TSpotifyKeysDao>>,
) -> Result<(), anyhow::Error>
where
    TSpotifyKeysDao: SpotifyKeyDao,
{
    spotify_keys_dao.lock().unwrap().clear()?;

    let mut response = request.into_response(302, None, &[("Location", redirect_uri)])?;
    response.flush()?;

    Ok(())
}
