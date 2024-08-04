use super::LedArray;

use anyhow::Result;

pub trait LedDisplayWrite {
    fn output_to_display(&mut self, array: &impl LedArray) -> Result<()>;
}
