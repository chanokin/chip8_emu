use minifb::{Key, Window, WindowOptions};
use crate::constants::{DISPLAY_WIDTH, DISPLAY_HEIGHT};


pub struct GUI {
    window: Window
}


impl GUI {
    pub fn new(scale: usize, fps: usize) -> Self {
        let mut window = Window::new(
                "Rust Chip-8 Emulator",
                DISPLAY_WIDTH * scale, // Scale up so it's not tiny
                DISPLAY_HEIGHT * scale,
                WindowOptions::default(),
            ).unwrap();

        window.set_target_fps(fps);

        GUI {
            window
        }
    }

    pub fn keep_running(&self) -> bool {
        self.window.is_open() && !self.window.is_key_down(Key::Escape)
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }

    pub fn is_key_down(&self, key: Key) -> bool {
        self.window.is_key_down(key)
    }

    pub fn key_state(&self) -> [bool; 16] {
        let mut keys_pressed = [false; 16];
        if self.window.is_key_down(Key::Key1) { keys_pressed[0x1] = true; }
        if self.window.is_key_down(Key::X)    { keys_pressed[0x0] = true; }
        // ... map the rest of the 16 keys ...
        keys_pressed
    }

    pub fn update_with_buffer(&mut self, buffer: &[u32]) {
        self.window.update_with_buffer(buffer, DISPLAY_WIDTH, DISPLAY_HEIGHT).unwrap();
    }

}

