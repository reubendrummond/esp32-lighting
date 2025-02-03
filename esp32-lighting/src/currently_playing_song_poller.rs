use std::{
    io::Read,
    sync::{Arc, Mutex},
    time::Duration,
};

use common::spotify::CurrentSong;
use esp_idf_svc::http::{
    client::{self, EspHttpConnection},
    Method,
};

use crate::dao::{
    current_song::{CurrentSongDao, SaveResult},
    spotify_key::SpotifyKeyDao,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCurrentlyPlayingTrackResponse {
    pub timestamp: u64,
    pub progress_ms: u64,
    pub is_playing: bool,
    pub item: Track,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Track {
    pub album: Album,
    pub duration_ms: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Album {
    pub images: Vec<Image>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    pub url: String,
    pub height: u32,
    pub width: u32,
}

static SPOTIFY_GET_CURRENTLY_PLAYING_TRACK_URL: &str =
    "https://api.spotify.com/v1/me/player/currently-playing";

pub struct CurrentlyPlayingSongPoller {
    current_song_dao: Arc<Mutex<dyn CurrentSongDao>>,
    spotify_key_dao: Arc<Mutex<dyn SpotifyKeyDao>>,
    client_connection: client::EspHttpConnection,
    pub poll_interval: Duration,
}

struct ConnectionReader<'a> {
    connection: &'a mut EspHttpConnection,
}

impl<'a> ConnectionReader<'a> {
    pub fn new(connection: &'a mut EspHttpConnection) -> Self {
        Self { connection }
    }
}

impl<'a> Read for ConnectionReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self.connection.read(buf) {
            Ok(size) => Ok(size),
            Err(e) => {
                log::error!("Failed to read from connection: {}", e);
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed to read from connection",
                ))
            }
        }
    }
}

impl CurrentlyPlayingSongPoller {
    pub fn new(
        current_song_dao: Arc<Mutex<dyn CurrentSongDao>>,
        spotify_key_dao: Arc<Mutex<dyn SpotifyKeyDao>>,
        client_connection: client::EspHttpConnection,
        poll_interval: Duration,
    ) -> Self {
        Self {
            current_song_dao,
            spotify_key_dao,
            client_connection,
            poll_interval,
        }
    }

    pub fn blocking_poll(&mut self) -> ! {
        loop {
            match self.poll_once() {
                Ok(_) => {}
                Err(e) => {
                    log::error!("Failed to poll: {}", e);
                }
            }
            std::thread::sleep(self.poll_interval);
        }
    }

    pub fn poll_once(&mut self) -> Result<(), anyhow::Error> {
        let spotify_key_dao = self.spotify_key_dao.lock().unwrap();
        let spotify_authorization_key = match spotify_key_dao.get_spotify_key() {
            Some(key) => key,
            None => {
                log::info!("No Spotify key found, skipping poll");
                return Ok(());
            }
        };

        self.client_connection.initiate_request(
            Method::Get,
            SPOTIFY_GET_CURRENTLY_PLAYING_TRACK_URL,
            &[(
                "Authorization",
                &format!("Bearer {}", spotify_authorization_key.access_token),
            )],
        )?;
        self.client_connection.initiate_response()?;

        let status = self.client_connection.status();

        match status {
            204 => {
                log::info!("No content");
                Ok(())
            }
            200 => {
                let connection_reader = ConnectionReader::new(&mut self.client_connection);
                let stream_deserializer = serde_json::Deserializer::from_reader(connection_reader);

                for result in stream_deserializer.into_iter::<GetCurrentlyPlayingTrackResponse>() {
                    match result {
                        Ok(response) => {
                            log::info!("Response: {:?}", response);

                            let album_url = response
                                .item
                                .album
                                .images
                                .iter()
                                .min_by_key(|image| image.width)
                                .map(|image| image.url.clone());

                            match album_url {
                                Some(album_url) => {
                                    match self
                                        .current_song_dao
                                        .lock()
                                        .unwrap()
                                        .save_current_song(CurrentSong { album_url })
                                        .unwrap()
                                    {
                                        SaveResult::Unchanged => {}
                                        SaveResult::Updated => {
                                            log::info!("Current song updated");
                                        }
                                    }
                                }
                                None => log::error!("No album URL found"),
                            }
                        }
                        Err(e) => {
                            log::error!("Failed to deserialize response: {}", e);
                            // Err(anyhow::anyhow!("Failed to deserialize response: {}", e))
                        }
                    };
                }

                Ok(())
            }
            _ => {
                log::error!("Failed to get currently playing track. Status: {}", status);
                Err(anyhow::anyhow!(
                    "Failed to get currently playing track. Status: {}",
                    status
                ))
            }
        }
    }
}
