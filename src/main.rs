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
    http::{server::EspHttpServer, Method},
    io::Write,
    nvs::EspDefaultNvsPartition,
    timer::EspTaskTimerService,
};
mod html;
mod wifi;
use html::{index, IndexProps};
use http::Uri;
use url::{form_urlencoded, Url};
use wifi::init_wifi;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take().unwrap();
    let timer_service = EspTaskTimerService::new().unwrap();
    let _wifi = match init_wifi(
        peripherals.modem,
        sysloop,
        Some(EspDefaultNvsPartition::take().unwrap()),
        timer_service,
    ) {
        Ok(wifi) => wifi,
        Err(e) => {
            log::error!("Failed to initialize wifi: {:?}", e);
            return;
        }
    };

    let led = Arc::new(Mutex::new(
        PinDriver::output(peripherals.pins.gpio2.downgrade_output()).unwrap(),
    ));

    let mut server = EspHttpServer::new(&Default::default()).unwrap();

    server
        .fn_handler("/", Method::Get, move |request| {
            log::info!("Request to {}", request.uri());

            let uri = request.uri();
            let uri = uri.parse::<Uri>()?;

            let query = uri.query();

            let light = match query {
                Some(query) => form_urlencoded::parse(query.as_bytes())
                    .find(|(key, _)| key == "light")
                    .and_then(|(_, value)| {
                        if value == "on" {
                            Some(true)
                        } else if value == "off" {
                            Some(false)
                        } else {
                            None
                        }
                    })
                    .unwrap_or(false),
                None => false,
            };

            if light {
                led.lock().unwrap().set_high()?;
            } else {
                led.lock().unwrap().set_low()?;
            }

            let html: String = index(IndexProps { light }).into();

            let mut response = request.into_ok_response()?;
            response.write_all(html.as_bytes())?;

            Ok::<(), anyhow::Error>(())
        })
        .unwrap();

    loop {
        log::info!("looping...");
        sleep(Duration::from_secs(2));
    }
}
