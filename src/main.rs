
use std::io::{self, Write};
use chip8_emu::App;

fn main() -> Result<(), String> {

    println!("\n\nStarting CHIP-8 Emulator...\n");
    println!("--> Press 'o' key to open a file dialog and open a rom");
    println!("--> Press 'i' key to reset the emulator (clears everything).");
    println!(concat!(
        "\nKeyboard mapping:\n",
        "\t 1 2 3 C                                   1 2 3 4\n",
        "\t 4 5 6 D    This is emulated with these    Q W E R\n",
        "\t 7 8 9 E    keyboard keys -->              A S D F\n",
        "\t A 0 B F                                   Z X C V\n"
    ));
    io::stdout().flush().unwrap();

    let mut app = App::new(10, 60); // scale 10x, 60 FPS

    app.run()?;


    Ok(())
}
