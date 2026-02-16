use crate::Machine;
use crate::GUI;
use crate::rom_reader::Reader;
use crate::constants::{DISPLAY_WIDTH, DISPLAY_HEIGHT};
pub struct App {
    machine: Machine,
    gui: GUI,
}


impl App {
    pub fn new(scale: usize, fps: usize) -> Self {
        App {
            machine: Machine::new(),
            gui: GUI::new(scale, fps),
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        self.machine.reset();
        // let mut rom =
        let mut rom_loaded = false; //

        let mut buffer: Vec<u32> = vec![0; DISPLAY_WIDTH * DISPLAY_HEIGHT];
        let mut loop_index = 0;

        while self.gui.keep_running() {
            // --- INPUT HANDLING ---
            if self.gui.trigger_open_rom() {
                match self.gui.load_rom_from_dialog() {
                    Ok(path) => {
                        // Here you would load the ROM into your machine
                        println!("Selected ROM: {:?}", path);
                        let program = Reader::read_rom(&path).expect("should be able to read rom");
                        self.machine.load_program(&program).expect("Failed to load ROM into machine");
                        println!("ROM loaded successfully!");
                        rom_loaded = true;
                    },
                    Err(e) => println!("Failed to load ROM: {}", e),
                }
            }

            if self.gui.trigger_machine_reset() {
                rom_loaded = false;
                self.machine.reset();
                println!("Machine reset!");
            }

            // Chip-8 uses a 16-key pad. Check state of each:
            let keys_pressed = self.gui.key_state();

            if rom_loaded {
                self.machine.step(keys_pressed)
                    .expect(&format!("\n\nFailed to execute step {}\n\t", loop_index)); // Handle errors properly in production code

                buffer = self.machine.frame_buffer();

            } else {
                for (i, pixel) in buffer.iter_mut().enumerate() {
                    *pixel = if i % 2 == 0 { 0xFFFFFF } else { 0x000000 }; // Example pattern
                }
            }

            self.gui.update_with_buffer(&buffer).expect("Failed to update GUI buffer");

            loop_index += 1;
        }

        Ok(())
    }
}