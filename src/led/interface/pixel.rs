use rgb::RGB8;

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

impl From<&Pixel> for RGB8 {
    fn from(pixel: &Pixel) -> Self {
        RGB8 {
            r: pixel.r,
            g: pixel.g,
            b: pixel.b,
        }
    }
}
