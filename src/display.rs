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

pub struct Display {
    // pixels: [bool; DISPLAY_WIDTH * DISPLAY_HEIGHT],
    ram: Box<Option<Ram>>,
}

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



    pub fn clear(&mut self) -> Result<(), DisplayError> {
        let mut _ram = match &mut *self.ram {
            Some(ram) => ram,
            None => return Err(DisplayError::MemoryNotInitialized),
        };

        for index in  DISPLAY_START_ADDRESS..DISPLAY_END_ADDRESS {
            let _result = _ram.set(index, 0);
            if _result.is_err() {
                return Err(DisplayError::ClearFailed);
            }
        }

        Ok(())

    }

    pub fn set_pixel(&mut self, x: usize, y: usize, value: bool) -> Result<(), DisplayError> {
        if x >= DISPLAY_WIDTH || y >= DISPLAY_HEIGHT {
            return Err(DisplayError::CoordinateError(x, y));
        }

        let ram = match &mut *self.ram {
            Some(ram) => ram,
            None => return Err(DisplayError::MemoryNotInitialized),
        };

        let (byte_index, bit_index) = coord_to_indices(x, y)?;

        let current_value = match ram.get(byte_index) {
            Ok(val) => val,
            Err(_) => return Err(DisplayError::CoordinateError(x, y)),
        };

        let new_value = if value {
            current_value | (1 << bit_index) // Set the pixel to 1
        } else {
            current_value & !(1 << bit_index) // Set the pixel to 0
        };

        match ram.set(byte_index, new_value) {
            Ok(_) => Ok(()),
            Err(_) => Err(DisplayError::SetPixelError(x, y))
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Result<bool, DisplayError> {
        if x >= DISPLAY_WIDTH || y >= DISPLAY_HEIGHT {
            return Err(DisplayError::CoordinateError(x, y));
        }

        let ram = match &*self.ram {
            Some(ram) => ram,
            None => return Err(DisplayError::MemoryNotInitialized),
        };

        let (byte_index, bit_index) = coord_to_indices(x, y)?;

        let current_value = match ram.get(byte_index) {
            Ok(val) => val,
            Err(_) => return Err(DisplayError::GetPixelError(x, y)),
        };

        let pixel_value = current_value >> bit_index;

        Ok(pixel_value == 1)

    }

    pub fn flip_pixel(&mut self, x: usize, y: usize) -> Result<bool, DisplayError> {
        if x >= DISPLAY_WIDTH || y >= DISPLAY_HEIGHT {
            return Err(DisplayError::CoordinateError(x, y));
        }

        let current_value = self.get_pixel(x, y)?;

        self.set_pixel(x, y, !current_value)?;

        Ok(!current_value)
    }

}
