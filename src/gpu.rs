use crate::constants::{
    DISPLAY_WIDTH,
    DISPLAY_HEIGHT,
    DISPLAY_CELLS_PER_ROW,
    DISPLAY_COLS_PER_CELL,
    DISPLAY_START_ADDRESS,
    DISPLAY_END_ADDRESS,
};

use crate::errors::DisplayError;
use crate::ram::Ram;

pub struct Gpu;

pub fn coord_to_indices(x: usize, y: usize) -> Result<(usize, usize), DisplayError> {
    if x >= DISPLAY_WIDTH || y >= DISPLAY_HEIGHT {
        return Err(DisplayError::CoordinateError(x, y));
    }

    let cell_for_row = y * DISPLAY_CELLS_PER_ROW;
    let cell_at_row = x / DISPLAY_COLS_PER_CELL;
    let byte_index = DISPLAY_START_ADDRESS + cell_for_row + cell_at_row;
    let bit_index = x % DISPLAY_COLS_PER_CELL;

    if byte_index >= DISPLAY_END_ADDRESS {
        return Err(DisplayError::OutOfBounds);
    }

    Ok((byte_index, bit_index))
}

impl Gpu {

    pub fn new() -> Self {
        Gpu{}
    }

    pub fn reset(&mut self, ram: &mut Ram) -> Result<(), DisplayError> {
        self.clear(ram)?;

        Ok(())
    }

    pub fn clear(&mut self, ram: &mut Ram) -> Result<(), DisplayError> {
        for index in  DISPLAY_START_ADDRESS..DISPLAY_END_ADDRESS {
            let _result = ram.set(index, 0);
            if _result.is_err() {
                return Err(DisplayError::ClearFailed);
            }
        }

        Ok(())

    }

    pub fn set_pixel(&mut self, x: usize, y: usize, value: u8, ram: &mut Ram) -> Result<(), DisplayError> {
        if x >= DISPLAY_WIDTH || y >= DISPLAY_HEIGHT {
            return Err(DisplayError::CoordinateError(x, y));
        }

        let (byte_index, bit_index) = coord_to_indices(x, y)?;

        let current_value = match ram.get(byte_index) {
            Ok(val) => val,
            Err(_) => return Err(DisplayError::CoordinateError(x, y)),
        };

        let new_value = if value == 1{
            current_value | (1 << bit_index) // Set the pixel to 1
        } else {
            current_value & !(1 << bit_index) // Set the pixel to 0
        };

        match ram.set(byte_index, new_value) {
            Ok(_) => Ok(()),
            Err(_) => Err(DisplayError::SetPixelError(x, y))
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize, ram: &Ram) -> Result<u8, DisplayError> {
        if x >= DISPLAY_WIDTH || y >= DISPLAY_HEIGHT {
            return Err(DisplayError::CoordinateError(x, y));
        }

        let (byte_index, bit_index) = coord_to_indices(x, y)?;

        let current_value = match ram.get(byte_index) {
            Ok(val) => val,
            Err(_) => return Err(DisplayError::GetPixelError(x, y)),
        };

        let pixel_value = (current_value >> bit_index) & 1;

        Ok(pixel_value)
    }

    pub fn flip_pixel(&mut self, x: usize, y: usize, ram: &mut Ram) -> Result<u8, DisplayError> {
        if x >= DISPLAY_WIDTH || y >= DISPLAY_HEIGHT {
            return Err(DisplayError::CoordinateError(x, y));
        }

        let current_value = self.get_pixel(x, y, ram)?;

        self.set_pixel(x, y, !current_value, ram)?;

        Ok(!current_value)
    }

    pub fn draw_sprite_at(&mut self, x: usize, y: usize, sprite_address: usize, sprite_height: u8, ram: &mut Ram) -> Result<bool, DisplayError> {
        // the bool in the return type indicates that we need to set VF as 1 because
        // a screen pixel was on and the sprite pixel was also on

        if x >= DISPLAY_WIDTH || y >= DISPLAY_HEIGHT {
            return Err(DisplayError::CoordinateError(x, y));
        }

        let sprite: Vec<u8> = match ram.get_range(sprite_address, sprite_height as usize) {
            Ok(data) => data,
            Err(e) => return Err(DisplayError::MemoryError(e)), // Handle memory error
        };

        let mut set_vf_to_true = false;
        for (row_index, &byte) in sprite.iter().enumerate() {
            if row_index + y >= DISPLAY_HEIGHT {
                break; // stop if we are out of bounds vertically
            }

            for bit_index in 0..8 {
                let sprite_pixel_value = (byte >> (7 - bit_index)) & 1;
                let pixel_x = x + bit_index;
                let pixel_y = y + row_index;

                if pixel_x >= DISPLAY_WIDTH {
                    continue; // Skip out-of-bounds pixels
                }

                let current_pixel_value = self.get_pixel(pixel_x, pixel_y, ram)?;

                if current_pixel_value > 0 && sprite_pixel_value == 1{
                    // If the pixel is already set and we are trying to set it again, we will flip it
                    self.set_pixel(pixel_x, pixel_y, 0, ram)?;
                    // self.flip_pixel(pixel_x, pixel_y, ram)?;
                    set_vf_to_true = true; // Set VF to 1 because we flipped a pixel that was already on
                } else {
                    // Otherwise, we just set the pixel
                    self.set_pixel(pixel_x, pixel_y, sprite_pixel_value, ram)?;
                }
            }
        }

        Ok(set_vf_to_true)
    }

}
