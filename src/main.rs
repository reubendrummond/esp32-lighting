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

use esp32_lighting::html::{index, IndexProps};
// use esp32_lighting::led;
use esp32_lighting::wifi::init_wifi;

use http::Uri;
use url::form_urlencoded;

use smart_leds_trait::{SmartLedsWrite, White};
use ws2812_esp32_rmt_driver::driver::color::LedPixelColorGrb24;
use ws2812_esp32_rmt_driver::{LedPixelEsp32Rmt, RGB8};

#[derive(Clone)]
struct RGBA {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

const NUM_BRIGHTNESS_LEVELS: usize = 6;
const MAPPING: [u8; NUM_BRIGHTNESS_LEVELS] = [0, 6, 30, 100, 254, 255];

fn map_to_brightness(c: u8, a: u8) -> u8 {
    let index = if a as usize >= NUM_BRIGHTNESS_LEVELS {
        NUM_BRIGHTNESS_LEVELS - 1
    } else {
        a as usize
    };

    (c as u16 * MAPPING[index] as u16 / 255) as u8
}

impl From<(u8, u8, u8, u8)> for RGBA {
    fn from((r, g, b, a): (u8, u8, u8, u8)) -> Self {
        RGBA { r, g, b, a }
    }
}
impl From<(u8, u8, u8)> for RGBA {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        RGBA {
            r,
            g,
            b,
            a: std::u8::MAX,
        }
    }
}

impl From<RGBA> for RGB8 {
    fn from(rgba: RGBA) -> Self {
        let r = map_to_brightness(rgba.r, rgba.a);
        let g = map_to_brightness(rgba.g, rgba.a);
        let b = map_to_brightness(rgba.b, rgba.a);
        RGB8 { r, g, b }
    }
}

// fn main() -> ! {
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

    // led test tings

    let led_pin = peripherals.pins.gpio26;
    let channel = peripherals.rmt.channel0;
    let mut ws2812 = LedPixelEsp32Rmt::<RGB8, LedPixelColorGrb24>::new(channel, led_pin).unwrap();
    const NUM_LEDS: usize = 16 * 16;

    let pixels =
        (0..NUM_LEDS).map(|i| RGBA::from(((i * std::u8::MAX as usize / NUM_LEDS) as u8, 0, 0)));
    ws2812.write(pixels).unwrap();
    loop {
        sleep(Duration::from_millis(1000));

        for brightness_level in 0..NUM_BRIGHTNESS_LEVELS {
            // let pixels = std::iter::repeat(RGBA::from((255, 0, 0, i as u8))).take(NUM_LEDS);
            let pixels = (0..NUM_LEDS).map(|i| {
                let level = (i * 255 / NUM_LEDS) as u8;
                RGBA::from((level, level, level, brightness_level as u8))
            });
            ws2812.write(pixels).unwrap();
            log::info!("BRIGHTNESS {}", brightness_level);
            sleep(Duration::from_millis(1000));
        }

        let pixels = std::iter::repeat(RGBA::from((6, 0, 0))).take(NUM_LEDS);
        ws2812.write(pixels).unwrap();
        log::info!("RED");
        sleep(Duration::from_millis(1000));

        let pixels = std::iter::repeat(RGB8::from((0, 6, 0))).take(NUM_LEDS);
        ws2812.write(pixels).unwrap();
        log::info!("GREEN");
        sleep(Duration::from_millis(1000));

        let pixels = std::iter::repeat(RGB8::from((0, 0, 6))).take(NUM_LEDS);
        ws2812.write(pixels).unwrap();
        log::info!("BLUE");
        sleep(Duration::from_millis(1000));

        let pixels = std::iter::repeat(RGB8::from((200, 200, 200))).take(NUM_LEDS);
        ws2812.write(pixels).unwrap();
        log::info!("WHITE");
        sleep(Duration::from_millis(1000));

        let pixels = std::iter::repeat(RGB8::from((0, 0, 0))).take(NUM_LEDS);
        ws2812.write(pixels).unwrap();
        log::info!("OFF");
        sleep(Duration::from_millis(1000));
    }
}
