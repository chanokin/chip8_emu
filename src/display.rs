use crate::constants::{
    DISPLAY_WIDTH,
    DISPLAY_HEIGHT,
    DISPLAY_START_ADDRESS,
    DISPLAY_END_ADDRESS,
    MemoryType,
};

use crate::errors::DisplayError;
use crate::ram::Ram;


// todo: implement with bit-fields (maybe?)
pub struct Display {
    // pixels: [bool; DISPLAY_WIDTH * DISPLAY_HEIGHT],
    ram: Box<Option<Ram>>,
}

impl Display {
    pub fn new() -> Self {
        Display {
            ram: Box::new(None),
        }
    }

    pub fn with_ram(ram: Ram) -> Self {
        Display {
            ram: Box::new(Some(ram)),
        }
    }

    pub fn clear(&mut self) {
        for index in  DISPLAY_START_ADDRESS..DISPLAY_END_ADDRESS{
            if let Some(ram) = &mut *self.ram {
                ram.set(index, 0).expect("Failed to clear display");
            } else {
                panic!("Display RAM is not initialized");
            }
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, value: bool) -> Result<(), DisplayError> {
        if x >= DISPLAY_WIDTH || y >= DISPLAY_HEIGHT {
            return Err(DisplayError::CoordinateError(x, y));
        }

        let byte_index = y * DISPLAY_WIDTH + DISPLAY_START_ADDRESS;

        if let Some(ram) = &mut *self.ram {
            let current_value = match ram.get(byte_index) {
                Ok(val) => val,
                Err(_) => return Err(DisplayError::CoordinateError(x, y)),
            };

            let new_value = if value { 1 } else { 0 };

            ram.set(byte_index, new_value);

        } else {
            panic!("Display RAM is not initialized");
        }

        Ok(())
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Result<bool, DisplayError> {
        if x >= DISPLAY_WIDTH || y >= DISPLAY_HEIGHT {
            return Err(DisplayError::CoordinateError(x, y));
        }

        let index = y * DISPLAY_WIDTH + x;

        Ok(self.pixels[index])

    }

    pub fn flip_pixel(&mut self, x: usize, y: usize) -> Result<bool, DisplayError> {
        if x >= DISPLAY_WIDTH || y >= DISPLAY_HEIGHT {
            return Err(DisplayError::CoordinateError(x, y));
        }

        let current_value = self.get_pixel(x, y)?;

        self.set_pixel(x, y, !current_value);

        Ok(!current_value)
    }

}
