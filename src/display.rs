use crate::constants::{
    DISPLAY_WIDTH,
    DISPLAY_HEIGHT,
};

use crate::errors::DisplayError;

// todo: implement with bit-fields (maybe?)
pub struct Display {
    pixels: [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
}

impl Display {
    pub fn new() -> Self {
        Display {
            pixels: [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
        }
    }

    pub fn clear(&mut self) {
        for row in self.pixels.iter_mut() {
            for pixel in row.iter_mut() {
                *pixel = false;
            }
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, value: bool) -> Result<(), DisplayError> {
        if x >= DISPLAY_WIDTH || y >= DISPLAY_HEIGHT {
            return Err(DisplayError::CoordinateError(x, y));
        }

        self.pixels[y][x] = value;

        Ok(())
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Result<bool, DisplayError> {
        if x >= DISPLAY_WIDTH || y >= DISPLAY_HEIGHT {
            return Err(DisplayError::CoordinateError(x, y));
        }

        Ok(self.pixels[y][x])

    }

    pub fn flip_pixel(&mut self, x: usize, y: usize) -> Result<bool, DisplayError> {
        if x >= DISPLAY_WIDTH || y >= DISPLAY_HEIGHT {
            return Err(DisplayError::CoordinateError(x, y));
        }

        let current_value = self.pixels[y][x];
        self.pixels[y][x] = !current_value;

        Ok(!current_value)
    }

}
