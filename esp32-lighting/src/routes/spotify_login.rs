use esp_idf_svc::http::server::Handler;
use esp_idf_svc::http::server::{EspHttpConnection, Request, Response};
use futures::ready;
use log::info;
// use rand::Rng;
use std::collections::HashMap;
use url::form_urlencoded;

const SPOTIFY_AUTH_URL: &str = "https://accounts.spotify.com/authorize";
const SPOTIFY_SCOPES: &str = "user-read-currently-playing";

// Helper function to generate a random state string for CSRF protection
// fn generate_random_state(length: usize) -> String {
//     let mut rng = rand::thread_rng();
//     let state: String = (0..length)
//         .map(|_| rng.gen_range(b'a', b'z') as char)
//         .collect();
//     state
// }

// Function to initiate the Spotify login
pub fn spotify_login_handler(
    request: Request<&mut EspHttpConnection>,
    client_id: &str,
    redirect_uri: &str,
) -> Result<(), anyhow::Error> {
    // Generate a random state to prevent CSRF attacks
    // let state = generate_random_state(16);

    // Build the Spotify login URL with necessary query parameters
    let mut params = HashMap::new();
    params.insert("response_type", "code");
    params.insert("client_id", client_id);
    params.insert("redirect_uri", redirect_uri);
    params.insert("scope", SPOTIFY_SCOPES);
    // params.insert("state", &state);

    // Encode the parameters into a query string
    let query_string = form_urlencoded::Serializer::new(String::new())
        .extend_pairs(params)
        .finish();

    // Redirect the user to the Spotify authorization URL
    let redirect_url = format!("{}?{}", SPOTIFY_AUTH_URL, query_string);
    // Assuming the `request` object allows sending a redirect response

    info!("Redirecting to Spotify login page: {}", redirect_url);

    // let mut response = request.into_ok_response()?;

    let mut response = request.into_response(302, None, &[("Location", &redirect_url)])?;
    response.flush()?;

    Ok(())
}
