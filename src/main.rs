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

use esp32_lighting::wifi::init_wifi;
use esp32_lighting::{
    html::{index, IndexProps},
    led::interface::{self, LedDisplayWrite, Pixel},
};

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

    // let pixels = (0..NUM_LEDS)
    //     .map(|i| RGBA8::from((255, 0, 0, (i * std::u8::MAX as usize / NUM_LEDS) as u8)));
    // ws2812.write(pixels).unwrap();

    // loop {
    //     sleep(Duration::from_millis(1000));

    //     // for brightness_level in 0..NUM_BRIGHTNESS_LEVELS {
    //     //     // let pixels = std::iter::repeat(RGBA::from((255, 0, 0, i as u8))).take(NUM_LEDS);
    //     //     let pixels = (0..NUM_LEDS).map(|i| {
    //     //         let level = (i * 255 / NUM_LEDS) as u8;
    //     //         RGBA8::from((level, level, level, brightness_level as u8))
    //     //     });
    //     //     ws2812.write(pixels).unwrap();
    //     //     log::info!("BRIGHTNESS {}", brightness_level);
    //     //     sleep(Duration::from_millis(1000));
    //     // }

    //     let pixels = std::iter::repeat(RGBA8::from((6, 0, 0, 255))).take(NUM_LEDS);
    //     ws2812.write(pixels).unwrap();
    //     log::info!("RED");
    //     sleep(Duration::from_millis(1000));

    //     let pixels = std::iter::repeat(RGBA8::from((0, 6, 0, 255))).take(NUM_LEDS);
    //     ws2812.write(pixels).unwrap();
    //     log::info!("GREEN");
    //     sleep(Duration::from_millis(1000));

    //     let pixels = std::iter::repeat(RGBA8::from((0, 0, 6, 255))).take(NUM_LEDS);
    //     ws2812.write(pixels).unwrap();
    //     log::info!("BLUE");
    //     sleep(Duration::from_millis(1000));

    //     let pixels = std::iter::repeat(RGBA8::from((200, 200, 200, 255))).take(NUM_LEDS);
    //     ws2812.write(pixels).unwrap();
    //     log::info!("WHITE");
    //     sleep(Duration::from_millis(1000));

    //     let pixels = std::iter::repeat(RGBA8::from((0, 0, 0, 255))).take(NUM_LEDS);
    //     ws2812.write(pixels).unwrap();
    //     log::info!("OFF");
    //     sleep(Duration::from_millis(1000));
    // }
}
