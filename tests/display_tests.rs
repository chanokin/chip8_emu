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
                assert_eq!(pixel_value.unwrap(), 0, "Pixel at ({}, {}) should be initialized to false", x, y);
            }
        }
    }


    #[test]
    fn display_set_pixel() {
        let mut ram = chip8_emu::Ram::new();
        let mut display = Gpu::new();

        display.set_pixel(0, 0, 1, &mut ram).expect("Failed to set pixel at (0, 0) to true");
        let value = display.get_pixel(0, 0, &mut ram).expect("Failed to get pixel at (0, 0)");
        assert_eq!(value, 1, "Pixel at (0, 0) should be set to true");
    }

    #[test]
    fn display_clear() {
        let mut ram = chip8_emu::Ram::new();
        let mut display = Gpu::new();
        display.set_pixel(0, 0, 1, &mut ram).expect("Failed to set pixel at (0, 0) to true");

        display.clear(&mut ram).expect("Failed to clear display");

        let value = display.get_pixel(0, 0, &ram).expect("Failed to get pixel at (0, 0)");
        assert_eq!(value, 0, "Pixel at (0, 0) should be cleared to false");
    }

    #[test]
    fn display_flip_pixel() {
        let mut ram = Ram::new();
        let mut display = Gpu::new();
        display.set_pixel(0, 0, 1, &mut ram).expect("Failed to set pixel at (0, 0) to true");
        let flipped_value = display.flip_pixel(0, 0, &mut ram).expect("Failed to flip pixel at (0, 0)");
        assert_eq!(flipped_value, 0, "Flipped pixel at (0, 0) should be false");
        let value = display.get_pixel(0, 0, &ram).expect("Failed to get pixel at (0, 0)");
        assert_eq!(value, 0, "Pixel at (0, 0) should be false after flipping");
    }

    #[test]
    fn draw_sprite() {
        let mut ram = Ram::new();
        let mut display = Gpu::new();
        display.reset(&mut ram).expect("Failed to initialize RAM for display");

        // Set a sprite in RAM
        let sprite_data = vec![0b1111_0000, 0b0000_1111]; // Example sprite
        let address = 0x20;

        let _ = ram.set_range(address, 2, sprite_data.as_slice())
            .expect("Failed to set sprite data in RAM");

        // Draw the sprite at (0, 0)
        display.draw_sprite_at(0, 0, address, 2, &mut ram).expect("Failed to draw sprite at (0, 0)");

        // Check first row
        assert_eq!(display.get_pixel(0, 0, &ram).unwrap(), 1);
        assert_eq!(display.get_pixel(1, 0, &ram).unwrap(), 1);
        assert_eq!(display.get_pixel(2, 0, &ram).unwrap(), 1);
        assert_eq!(display.get_pixel(3, 0, &ram).unwrap(), 1);
        assert_eq!(display.get_pixel(4, 0, &ram).unwrap(), 0);
        assert_eq!(display.get_pixel(5, 0, &ram).unwrap(), 0);
        assert_eq!(display.get_pixel(6, 0, &ram).unwrap(), 0);
        assert_eq!(display.get_pixel(7, 0, &ram).unwrap(), 0);
        // Check the second row
        assert_eq!(display.get_pixel(0, 1, &ram).unwrap(), 0);
        assert_eq!(display.get_pixel(1, 1, &ram).unwrap(), 0);
        assert_eq!(display.get_pixel(2, 1, &ram).unwrap(), 0);
        assert_eq!(display.get_pixel(3, 1, &ram).unwrap(), 0);
        assert_eq!(display.get_pixel(4, 1, &ram).unwrap(), 1);
        assert_eq!(display.get_pixel(5, 1, &ram).unwrap(), 1);
        assert_eq!(display.get_pixel(6, 1, &ram).unwrap(), 1);
        assert_eq!(display.get_pixel(7, 1, &ram).unwrap(), 1);
    }

}