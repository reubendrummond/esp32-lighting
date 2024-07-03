use super::Pixel;

pub trait LedArray {
    fn ordered_iter(&self) -> impl Iterator<Item = &Pixel>;
}
