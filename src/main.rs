
use std::io::{self, Write};
use chip8_emu::App;

fn main() -> Result<(), String> {

    println!("\n\nStarting CHIP-8 Emulator...\n");
    io::stdout().flush().unwrap();

    let mut app = App::new(10, 60); // scale 10x, 60 FPS

    app.run()?;


    Ok(())
}
