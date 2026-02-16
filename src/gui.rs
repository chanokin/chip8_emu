use minifb::{Key, Window, WindowOptions};
use crate::constants::{DISPLAY_WIDTH, DISPLAY_HEIGHT};
use dialog::DialogBox;



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
        if self.window.is_key_down(Key::Key1) { keys_pressed[0x0] = true; }
        if self.window.is_key_down(Key::Key2) { keys_pressed[0x1] = true; }
        if self.window.is_key_down(Key::Key3) { keys_pressed[0x2] = true; }
        if self.window.is_key_down(Key::Key4) { keys_pressed[0x3] = true; }

        if self.window.is_key_down(Key::Q)    { keys_pressed[0x4] = true; }
        if self.window.is_key_down(Key::W)    { keys_pressed[0x5] = true; }
        if self.window.is_key_down(Key::E)    { keys_pressed[0x6] = true; }
        if self.window.is_key_down(Key::R)    { keys_pressed[0x7] = true; }

        if self.window.is_key_down(Key::A)    { keys_pressed[0x8] = true; }
        if self.window.is_key_down(Key::S)    { keys_pressed[0x9] = true; }
        if self.window.is_key_down(Key::D)    { keys_pressed[0xA] = true; }
        if self.window.is_key_down(Key::F)    { keys_pressed[0xB] = true; }

        if self.window.is_key_down(Key::Z)    { keys_pressed[0xC] = true; }
        if self.window.is_key_down(Key::X)    { keys_pressed[0xD] = true; }
        if self.window.is_key_down(Key::C)    { keys_pressed[0xE] = true; }
        if self.window.is_key_down(Key::V)    { keys_pressed[0xF] = true; }

        keys_pressed
    }

    pub fn update_with_buffer(&mut self, buffer: &[u32]) -> Result<(), minifb::Error> {
        self.window.update_with_buffer(buffer, DISPLAY_WIDTH, DISPLAY_HEIGHT)?;
        Ok(())
    }


    pub fn trigger_open_rom(&self) -> bool {
        self.window.is_key_pressed(Key::O, minifb::KeyRepeat::No)
    }

    pub fn trigger_machine_reset(&self) -> bool {
        self.window.is_key_pressed(Key::I, minifb::KeyRepeat::No)
    }

    pub fn load_rom_from_dialog(&mut self) -> Result<String, String> {
        // Use rfd crate to open a file dialog
        let choice = dialog::FileSelection::new("Select a CHIP-8 ROM")
            .title("Open ROM")
            // .path("roms") // Start in the ROMs directory
            .show()
            .expect("Failed to open file dialog");

        match choice {
            Some(path) => {
                Ok(path)
            },
            None => Err("No file selected".to_string()),

        }
    }


}

