use chip8_emu::Gpu;
use chip8_emu::Ram;
use chip8_emu::constants::{DISPLAY_WIDTH, DISPLAY_HEIGHT};

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn display_initialization() {
        let mut ram = Ram::new();
        let mut display = Gpu::new();

        display.reset(&mut ram).expect("Failed to initialize RAM for display");
        for y in 0..DISPLAY_HEIGHT {
            for x in 0..DISPLAY_WIDTH {
                let pixel_value = display.get_pixel(x, y, & ram);
                assert!(pixel_value.is_ok(), "Failed to get pixel at ({}, {}): {}", x, y, pixel_value.unwrap_err());
                assert_eq!(pixel_value.unwrap(), false, "Pixel at ({}, {}) should be initialized to false", x, y);
            }
        }
    }


    #[test]
    fn display_set_pixel() {
        let mut ram = chip8_emu::Ram::new();
        let mut display = Gpu::new();

        display.set_pixel(0, 0, true, &mut ram).expect("Failed to set pixel at (0, 0) to true");
        let value = display.get_pixel(0, 0, &mut ram).expect("Failed to get pixel at (0, 0)");
        assert_eq!(value, true, "Pixel at (0, 0) should be set to true");
    }

    #[test]
    fn display_clear() {
        let mut ram = chip8_emu::Ram::new();
        let mut display = Gpu::new();
        display.set_pixel(0, 0, true, &mut ram).expect("Failed to set pixel at (0, 0) to true");

        display.clear(&mut ram).expect("Failed to clear display");

        let value = display.get_pixel(0, 0, &ram).expect("Failed to get pixel at (0, 0)");
        assert_eq!(value, false, "Pixel at (0, 0) should be cleared to false");
    }

    #[test]
    fn display_flip_pixel() {
        let mut ram = Ram::new();
        let mut display = Gpu::new();
        display.set_pixel(0, 0, true, &mut ram).expect("Failed to set pixel at (0, 0) to true");
        let flipped_value = display.flip_pixel(0, 0, &mut ram).expect("Failed to flip pixel at (0, 0)");
        assert_eq!(flipped_value, false, "Flipped pixel at (0, 0) should be false");
        let value = display.get_pixel(0, 0, &ram).expect("Failed to get pixel at (0, 0)");
        assert_eq!(value, false, "Pixel at (0, 0) should be false after flipping");
    }

}