use crate::constants::{
    DISPLAY_WIDTH,
    DISPLAY_HEIGHT,
};

// todo: implement with bitfields
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

    pub fn set_pixel(&mut self, x: usize, y: usize, value: bool) {
        if x < DISPLAY_WIDTH && y < DISPLAY_HEIGHT {
            self.pixels[y][x] = value;
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> bool {
        if x < DISPLAY_WIDTH && y < DISPLAY_HEIGHT {
            self.pixels[y][x]
        } else {
            false
        }
    }

}
