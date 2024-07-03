use super::{LedArray, Pixel};

pub struct LedRectangularArray {
    pub width: usize,
    pub height: usize,
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
