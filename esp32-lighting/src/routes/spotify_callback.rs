use std::collections::HashMap;
use std::fmt::format;
use std::sync::Arc;
use std::sync::Mutex;

use base64::Engine;
use common::spotify::SpotifyKey;
use esp_idf_svc::http::client;
use esp_idf_svc::http::server;
use esp_idf_svc::http::Method;
use http::Uri;
use log::info;
use serde::Deserialize;

use crate::dao::spotify_key::SpotifyKeyDao;
use crate::utils::get_query_value;
use crate::utils::url_encode_params;

fn construct_spotify_auth_form_data<'a>(
    auth_code: &'a str,
    redirect_uri: &'a str,
    // client_id: &'a str,
    // client_secret: &'a str,
) -> HashMap<&'a str, &'a str> {
    let mut params = HashMap::new();
    params.insert("code", auth_code);
    params.insert("redirect_uri", redirect_uri);
    params.insert("grant_type", "authorization_code");
    // params.insert("client_id", client_id);
    // params.insert("client_secret", client_secret);

    params
}

#[derive(Debug, Deserialize)]
struct SpotifyAuthResponse {
    access_token: String,
    token_type: String,
    scope: String,
    expires_in: u32,
    refresh_token: String,
}

pub fn spotify_callback_handler<TSpotifyKeysDao>(
    request: server::Request<&mut server::EspHttpConnection>,
    redirect_uri: &str,
    client_id: &str,
    client_secret: &str,
    mut client_connection: client::EspHttpConnection,
    buffer: Arc<Mutex<Vec<u8>>>,
    spotify_keys_dao: Arc<Mutex<TSpotifyKeysDao>>,
) -> Result<(), anyhow::Error>
where
    TSpotifyKeysDao: SpotifyKeyDao,
{
    let uri = request.uri();

    info!("Request to {}", uri);

    let uri = uri.parse::<Uri>()?;
    let query = match uri.query() {
        Some(query) => query,
        None => return Err(anyhow::anyhow!("No query found")),
    };

    let auth_code = match get_query_value(query, "code") {
        Some(auth_code) => auth_code,
        None => return Err(anyhow::anyhow!("No auth code found")),
    };

    let params = construct_spotify_auth_form_data(&auth_code, redirect_uri);

    info!("Auth code: {}", auth_code);

    let authorization = base64::engine::general_purpose::STANDARD
        .encode(format!("{}:{}", client_id, client_secret));
    let authorization = format!("Basic {}", authorization);
    info!("Authorization: {}", authorization);

    // request to client
    client_connection.initiate_request(
        Method::Post,
        "https://accounts.spotify.com/api/token",
        &[
            ("Content-Type", "application/x-www-form-urlencoded"),
            ("Authorization", authorization.as_str()),
        ],
    )?;
    let body = url_encode_params(&params);
    info!("Body: {}", body);
    client_connection.write_all(body.as_bytes()).unwrap();
    client_connection.initiate_response()?;

    let status = client_connection.status();
    let mut buffer = buffer.lock().unwrap();
    let body_size = client_connection.read(&mut buffer)?;

    info!("Status: {}", status);
    info!("Body size: {}", body_size);
    info!("Body: {:?}", &buffer[..body_size].utf8_chunks());

    match status {
        200 => (),
        _ => return Err(anyhow::anyhow!("Failed to get Spotify auth response")),
    };

    let spotify_response: SpotifyAuthResponse = match serde_json::from_slice(&buffer[..body_size]) {
        Ok(parsed) => parsed,
        Err(e) => {
            info!("Failed to parse JSON: {:?}", e);
            return Err(anyhow::anyhow!("Failed to parse Spotify auth response"));
        }
    };

    log::info!("Spotify response: {:?}", spotify_response);

    spotify_keys_dao
        .lock()
        .unwrap()
        .save_spotify_key(SpotifyKey {
            access_token: spotify_response.access_token,
            refresh_token: spotify_response.refresh_token,
            expires_at: spotify_response.expires_in,
        })?;

    let mut response = request.into_response(302, None, &[("Location", "/")])?;
    response.flush()?;

    Ok(())
}
