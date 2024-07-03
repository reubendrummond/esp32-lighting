use anyhow::Result;
use smart_leds_trait::SmartLedsWrite;

use crate::led::pixel::Pixel;

use super::{LedArray, LedDisplayWrite};

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
