use core::panic;
use std::{
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};

use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::{
        gpio::{OutputPin, PinDriver},
        peripherals::Peripherals,
    },
    http::{client::Configuration, server::EspHttpServer, Method},
    io::Write,
    nvs::EspDefaultNvsPartition,
    sys::esp_crt_bundle_attach,
    timer::EspTaskTimerService,
    wifi::{AsyncWifi, EspWifi},
};

use esp32_lighting::{
    currently_playing_song_poller::CurrentlyPlayingSongPoller,
    env::{SPOTIFY_CLIENT_ID, SPOTIFY_CLIENT_SECRET},
    routes::{self, spotify_login::spotify_login_handler},
    wifi::init_wifi,
};
use web::pages::{index, IndexProps};

use common::led::interface::{self, LedDisplayWrite};
use common::led::pixel::Pixel;

use http::Uri;
use rgb::RGB8;
use url::form_urlencoded;
use ws2812_esp32_rmt_driver::{driver::color::LedPixelColorGrb24, LedPixelEsp32Rmt};

fn get_redirect_uri(wifi: &AsyncWifi<EspWifi<'static>>) -> String {
    format!(
        "http://{}/spotify/callback",
        wifi.wifi().sta_netif().get_ip_info().unwrap().ip
    )
}

fn main() -> ! {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take().unwrap();
    let timer_service = EspTaskTimerService::new().unwrap();
    let wifi = match init_wifi(
        peripherals.modem,
        sysloop,
        Some(EspDefaultNvsPartition::take().unwrap()),
        timer_service,
    ) {
        Ok(wifi) => wifi,
        Err(e) => {
            panic!("Failed to initialize wifi: {:?}", e)
        }
    };

    let redirect_uri = &get_redirect_uri(&wifi);
    log::info!("Redirect URI: {}", redirect_uri);

    let led = Arc::new(Mutex::new(
        PinDriver::output(peripherals.pins.gpio2.downgrade_output()).unwrap(),
    ));
    let buffer = Arc::new(Mutex::new(vec![0; 1024]));
    let spotify_keys_dao = Arc::new(Mutex::new(
        esp32_lighting::dao::spotify_key::SpotifyKeyDaoImpl::new(),
    ));
    let current_song_dao = Arc::new(Mutex::new(
        esp32_lighting::dao::current_song::CurrentSongDaoImpl::new(),
    ));

    let mut server = EspHttpServer::new(&Default::default()).unwrap();

    server
        .fn_handler("/", Method::Get, |request| {
            routes::index::index_handler(
                request,
                led.clone(),
                spotify_keys_dao.clone(),
                current_song_dao.clone(),
            )
        })
        .unwrap()
        .fn_handler("/spotify/login", Method::Get, |request| {
            spotify_login_handler(request, SPOTIFY_CLIENT_ID, redirect_uri)
        })
        .unwrap()
        .fn_handler("/spotify/callback", Method::Get, |request| {
            let client_connection =
                esp_idf_svc::http::client::EspHttpConnection::new(&Configuration {
                    crt_bundle_attach: Some(esp_crt_bundle_attach),
                    ..Default::default()
                })
                .unwrap();

            routes::spotify_callback::spotify_callback_handler(
                request,
                redirect_uri,
                SPOTIFY_CLIENT_ID,
                SPOTIFY_CLIENT_SECRET,
                client_connection,
                buffer.clone(),
                spotify_keys_dao.clone(),
            )
        })
        .unwrap()
        .fn_handler("/spotify/logout", Method::Get, |request| {
            routes::spotify_logout::spotify_logout_handler(request, "/", spotify_keys_dao.clone())
        })
        .unwrap();

    // led test tings

    let led_pin = peripherals.pins.gpio26;
    let channel = peripherals.rmt.channel0;

    let ws2812 = LedPixelEsp32Rmt::<RGB8, LedPixelColorGrb24>::new(channel, led_pin).unwrap();
    let mut display = interface::LedDisplay::new(ws2812);

    let mut led_array = interface::LedRectangularArray::new(16, 16);

    // for y in 0..16 {
    //     for x in 0..16 {
    //         let r = (x * 255 / 16) as u8;
    //         let g = (y * 255 / 16) as u8;
    //         let b = 0;
    //         led_array.set_pixel(x, y, Pixel::new(r, g, b));
    //     }
    // }

    // display.output_to_display(&led_array).unwrap();

    let client_connection = esp_idf_svc::http::client::EspHttpConnection::new(&Configuration {
        crt_bundle_attach: Some(esp_crt_bundle_attach),
        ..Default::default()
    })
    .unwrap();

    CurrentlyPlayingSongPoller::new(
        current_song_dao.clone(),
        spotify_keys_dao.clone(),
        client_connection,
        Duration::from_secs(2),
    )
    .blocking_poll();
}
