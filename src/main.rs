
use std::io::{self, Write};
use chip8_emu::App;

fn main() -> Result<(), String> {

    println!("Hello, world!");
    io::stdout().flush().unwrap();

    let mut app = App::new(10, 60); // scale 10x, 60 FPS let mut buffer: Vec<u32> = vec![0; DISPLAY_WIDTH * DISPLAY_HEIGHT]; while app.gui.keep_running() { // --- INPUT HANDLING --- // Chip-8 uses a 16-key pad. Check state of each: let keys_pressed = app.gui.key_state(); // --- EMULATION STEP --- // Your_Chip8_Core.tick(keys_pressed); app.machine.step(keys_pressed).expect("Failed to execute step"); // Handle errors properly in production code // --- RENDER STEP --- // Convert your internal 1-bit/bool array to u32 (RGB) colors for (i, pixel) in buffer.iter_mut().enumerate() { *pixel = if i % 2 == 0 { 0xFFFFFF } else { 0x000000 }; // Example pattern } app.gui.update_with_buffer(&buffer).expect("Failed to update GUI buffer"); }

    app.run()?;


    Ok(())
}
