use std::collections::HashMap;

use esp_idf_svc::http::server::{EspHttpConnection, Request};
use http::Uri;
use log::info;

fn get_auth_code(query: &str) -> Option<&str> {
    None
}

fn construct_spotify_auth_form_data<'a>(
    auth_code: &'a str,
    redirect_uri: &'a str,
    client_id: &'a str,
    client_secret: &'a str,
) -> HashMap<&'a str, &'a str> {
    let mut params = HashMap::new();
    params.insert("code", auth_code);
    params.insert("redirect_uri", redirect_uri);
    params.insert("grant_type", "authorization_code");
    params.insert("client_id", client_id);
    params.insert("client_secret", client_secret);

    params
}

pub fn spotify_callback_handler(
    request: Request<&mut EspHttpConnection>,
    redirect_uri: &str,
    client_id: &str,
    client_secret: &str,
) -> Result<(), anyhow::Error> {
    let uri = request.uri();

    info!("Request to {}", uri);

    let uri = uri.parse::<Uri>()?;
    let query = match uri.query() {
        Some(query) => query,
        None => return Err(anyhow::anyhow!("No query found")),
    };

    let auth_code = match get_auth_code(query) {
        Some(auth_code) => auth_code,
        None => return Err(anyhow::anyhow!("No auth code found")),
    };

    let params =
        construct_spotify_auth_form_data(auth_code, redirect_uri, client_id, client_secret);

    // request to client

    Ok(())
}
