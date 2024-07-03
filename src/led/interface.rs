use anyhow::Result;

use smart_leds_trait::SmartLedsWrite;
use ws2812_esp32_rmt_driver::RGB8;

#[derive(Clone, Default)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Pixel { r, g, b }
    }
}

pub trait LedArray {
    fn ordered_iter(&self) -> impl Iterator<Item = &Pixel>;
}

pub struct LedRectangularArray {
    width: usize,
    height: usize,
    pixels: Vec<Vec<Pixel>>,
}

impl LedRectangularArray {
    pub fn new(width: usize, height: usize) -> Self {
        let pixels = vec![vec![Pixel::new(5, 10, 20); width]; height];
        // let pixels = vec![vec![Pixel::default(); width]; height];

        LedRectangularArray {
            width,
            height,
            pixels,
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, pixel: Pixel) {
        self.pixels[y][x] = pixel;
    }
}

impl LedArray for LedRectangularArray {
    fn ordered_iter(&self) -> impl Iterator<Item = &Pixel> {
        // let d = self.width * self.height;
        // TODO: make ordered from top left to bottom right
        self.pixels.iter().flat_map(|row| row.iter())
    }
}

pub trait LedDisplayWrite {
    fn output_to_display(&mut self, array: &impl LedArray) -> Result<()>;
}

pub struct LedDisplay<TDriver: SmartLedsWrite> {
    driver: TDriver,
}

impl<TDriver> LedDisplay<TDriver>
where
    TDriver: SmartLedsWrite,
    TDriver::Color: for<'a> From<&'a Pixel>,
    TDriver::Error: std::error::Error + Send + Sync + 'static,
{
    pub fn new(driver: TDriver) -> Self {
        LedDisplay { driver }
    }

    fn write(
        &mut self,
        colors: &mut impl std::iter::Iterator<Item = TDriver::Color>,
    ) -> Result<()> {
        self.driver.write(colors)?;
        Ok(())
    }
}

impl<TDisplay> LedDisplayWrite for LedDisplay<TDisplay>
where
    TDisplay: SmartLedsWrite,
    TDisplay::Color: for<'a> From<&'a Pixel>,
    TDisplay::Error: std::error::Error + Send + Sync + 'static,
{
    fn output_to_display(&mut self, array: &impl LedArray) -> Result<()> {
        let mut colors = array.ordered_iter().map(TDisplay::Color::from);
        self.write(&mut colors)?;
        Ok(())
    }
}

impl From<&Pixel> for RGB8 {
    fn from(pixel: &Pixel) -> Self {
        RGB8 {
            r: pixel.r,
            g: pixel.g,
            b: pixel.b,
        }
    }
}
