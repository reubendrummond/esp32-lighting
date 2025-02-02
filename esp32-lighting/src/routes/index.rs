use std::sync::{Arc, Mutex};

use esp_idf_svc::{
    hal::gpio::PinDriver,
    http::server::{EspHttpConnection, Request},
    io::Write,
};

use web::pages::{index, IndexProps};

use http::Uri;

use crate::utils::get_query_value;

pub fn index_handler(
    request: Request<&mut EspHttpConnection>,
    led: Arc<
        Mutex<PinDriver<'_, esp_idf_svc::hal::gpio::AnyOutputPin, esp_idf_svc::hal::gpio::Output>>,
    >,
) -> Result<(), anyhow::Error> {
    log::info!("Request to {}", request.uri());

    let uri = request.uri();
    let uri = uri.parse::<Uri>()?;

    let query = uri.query();

    let light = match query {
        Some(query) => get_query_value(query, "light")
            .map(|value| match value.as_str() {
                "on" => true,
                "off" => false,
                _ => false,
            })
            .unwrap_or(false),
        None => false,
    };

    match light {
        true => led.lock().unwrap().set_high()?,
        false => led.lock().unwrap().set_low()?,
    }

    let html: String = index(IndexProps { light }).into();

    let mut response = request.into_ok_response()?;
    response.write_all(html.as_bytes())?;

    Ok(())
}
