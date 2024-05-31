use std::{thread::sleep, time::Duration};

use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::peripherals::Peripherals,
    http::{server::EspHttpServer, Method},
    io::Write,
    nvs::EspDefaultNvsPartition,
    timer::EspTaskTimerService,
};
mod wifi;
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

    let mut server = EspHttpServer::new(&Default::default()).unwrap();

    server
        .fn_handler("/", Method::Get, move |request| {
            log::info!("Request to {}", request.uri());

            let html = templated_html("Welcome to ESP32 lighting!");

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

fn templated_html(content: impl AsRef<str>) -> String {
    format!(
        r#"
<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8">
        <title>ESP32 Ligting</title>
    </head>
    <body>
        {}
    </body>
</html>
"#,
        content.as_ref()
    )
}
