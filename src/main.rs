pub use chip8_emu::Machine;
use std::io::{self, Write};
use std::{thread, time::Duration};

use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

// fn get_key(device_state: &DeviceState) -> Result<Vec<Keycode>> {
//     let keys = device_state.get_keys();
//     Ok(keys)
// }

fn main(){

    println!("Hello, world!");
    io::stdout().flush().unwrap();


    let mut machine = Machine::new();
    machine.reset();

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Rust Chip-8 Emulator",
        WIDTH * 10, // Scale up so it's not tiny
        HEIGHT * 10,
        WindowOptions::default(),
    ).unwrap();

    // Limit to ~60fps for the display loop
    window.set_target_fps(60);


    while window.is_open() && !window.is_key_down(Key::Escape) {
        // --- INPUT HANDLING ---
        // Chip-8 uses a 16-key pad. Check state of each:
        let mut keys_pressed = [false; 16];
        if window.is_key_down(Key::Key1) { keys_pressed[0x1] = true; }
        if window.is_key_down(Key::X)    { keys_pressed[0x0] = true; }
        // ... map the rest of the 16 keys ...

        // --- EMULATION STEP ---
        // Your_Chip8_Core.tick(keys_pressed);

        // --- RENDER STEP ---
        // Convert your internal 1-bit/bool array to u32 (RGB) colors
        for (i, pixel) in buffer.iter_mut().enumerate() {
            *pixel = if i % 2 == 0 { 0xFFFFFF } else { 0x000000 }; // Example pattern
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }

}
