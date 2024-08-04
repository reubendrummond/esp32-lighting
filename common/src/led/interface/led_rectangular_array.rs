use crate::led::pixel::Pixel;
use either::Either::{Left, Right};

use super::LedArray;

#[cfg(test)]
mod tests;

pub struct LedRectangularArray {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Vec<Pixel>>,
}

impl LedRectangularArray {
    pub fn new(width: usize, height: usize) -> Self {
        let pixels = vec![vec![Pixel::default(); width]; height];

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
        self.pixels.iter().rev().enumerate().flat_map(|(i, row)| {
            if i % 2 == 0 {
                Left(row.iter())
            } else {
                Right(row.iter().rev())
            }
        })
    }
}
