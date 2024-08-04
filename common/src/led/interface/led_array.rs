use crate::led::pixel::Pixel;

pub trait LedArray {
    /// Returns an iterator over the pixels in the order they should be displayed according to the hardware.
    fn ordered_iter(&self) -> impl Iterator<Item = &Pixel>;
}
