use chip8_emu::Display;
use chip8_emu::constants::{DISPLAY_WIDTH, DISPLAY_HEIGHT};

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn display_initialization() {
        let display = Display::new();
        for y in 0..DISPLAY_HEIGHT {
            for x in 0..DISPLAY_WIDTH {
                assert_eq!(display.get_pixel(x, y).unwrap(), false, "Pixel at ({}, {}) should be initialized to false", x, y);
            }
        }
    }


    #[test]
    fn display_set_pixel() {
        let mut display = Display::new();
        display.set_pixel(0, 0, true).expect("Failed to set pixel at (0, 0) to true");
        let value = display.get_pixel(0, 0).expect("Failed to get pixel at (0, 0)");
        assert_eq!(value, true, "Pixel at (0, 0) should be set to true");
    }

    #[test]
    fn display_clear() {
        let mut display = Display::new();
        display.set_pixel(0, 0, true).expect("Failed to set pixel at (0, 0) to true");

        display.clear();

        let value = display.get_pixel(0, 0).expect("Failed to get pixel at (0, 0)");
        assert_eq!(value, false, "Pixel at (0, 0) should be cleared to false");
    }

    #[test]
    fn display_flip_pixel() {
        let mut display = Display::new();
        display.set_pixel(0, 0, true).expect("Failed to set pixel at (0, 0) to true");
        let flipped_value = display.flip_pixel(0, 0).expect("Failed to flip pixel at (0, 0)");
        assert_eq!(flipped_value, false, "Flipped pixel at (0, 0) should be false");
        let value = display.get_pixel(0, 0).expect("Failed to get pixel at (0, 0)");
        assert_eq!(value, false, "Pixel at (0, 0) should be false after flipping");
    }

}