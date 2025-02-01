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
    http::{server::EspHttpServer, Method},
    io::Write,
    nvs::EspDefaultNvsPartition,
    timer::EspTaskTimerService,
};

use esp32_lighting::{routes, wifi::init_wifi};
use web::pages::{index, IndexProps};

use common::led::interface::{self, LedDisplayWrite};
use common::led::pixel::Pixel;

use http::Uri;
use rgb::RGB8;
use url::form_urlencoded;
use ws2812_esp32_rmt_driver::{driver::color::LedPixelColorGrb24, LedPixelEsp32Rmt};

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
    let _wifi = match init_wifi(
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

    let led = Arc::new(Mutex::new(
        PinDriver::output(peripherals.pins.gpio2.downgrade_output()).unwrap(),
    ));

    let mut server = EspHttpServer::new(&Default::default()).unwrap();

    server
        .fn_handler("/", Method::Get, move |request| {
            routes::index::index_handler(request, Arc::clone(&led))
        })
        .unwrap();

    // led test tings

    let led_pin = peripherals.pins.gpio26;
    let channel = peripherals.rmt.channel0;

    let ws2812 = LedPixelEsp32Rmt::<RGB8, LedPixelColorGrb24>::new(channel, led_pin).unwrap();
    let mut display = interface::LedDisplay::new(ws2812);

    let mut led_array = interface::LedRectangularArray::new(16, 16);

    for y in 0..16 {
        for x in 0..16 {
            let r = (x * 255 / 16) as u8;
            let g = (y * 255 / 16) as u8;
            let b = 0;
            led_array.set_pixel(x, y, Pixel::new(r, g, b));
        }
    }

    display.output_to_display(&led_array).unwrap();

    loop {
        log::info!("Looping...");
        sleep(Duration::from_millis(1000));
    }
}
