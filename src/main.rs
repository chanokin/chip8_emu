pub use chip8_emu::Machine;
pub use chip8_emu::GUI;
use std::io::{self, Write};



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
    let mut gui = GUI::new(10, 60); // scale 10x, 60 FPS
    let mut loop_index = 0;
    while gui.keep_running() {
        // --- INPUT HANDLING ---
        // Chip-8 uses a 16-key pad. Check state of each:
        let keys_pressed = gui.key_state();
        // --- EMULATION STEP ---
        // Your_Chip8_Core.tick(keys_pressed);
        machine.step(keys_pressed).expect(&format!("Failed to execute step {}", loop_index)); // Handle errors properly in production code
        // --- RENDER STEP ---
        // Convert your internal 1-bit/bool array to u32 (RGB) colors
        for (i, pixel) in buffer.iter_mut().enumerate() {
            *pixel = if i % 2 == 0 { 0xFFFFFF } else { 0x000000 }; // Example pattern
        }

        gui.update_with_buffer(&buffer);
        loop_index += 1;
    }

}
