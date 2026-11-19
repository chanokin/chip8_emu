use crate::Machine;
use crate::GUI;
use crate::rom_reader::Reader;
use crate::constants::CYCLES_PER_FRAME;

pub struct App {
    machine: Machine,
    gui: GUI,
}



impl App {
    pub fn new(scale: usize, fps: usize) -> Self {
        App {
            machine: Machine::new(0f32, 0f32),
            gui: GUI::new(scale, fps),
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        let mut trigger_redraw: bool = true;

        self.machine.reset();
        // let mut rom =
        let mut rom_loaded = false; //

        // let mut buffer: &[u32] = ![0; DISPLAY_WIDTH * DISPLAY_HEIGHT];
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
                        trigger_redraw = true; // Force redraw to show the new ROM's initial state
                    },
                    Err(e) => println!("Failed to load ROM: {}", e),
                }
            }

            if self.gui.trigger_machine_reset() {
                rom_loaded = false;
                self.machine.reset();
                println!("Machine reset!");
                trigger_redraw = true; // Force redraw to show the reset state
            }

            // Chip-8 uses a 16-key pad. Check state of each:
            let keys_pressed = self.gui.key_state();

            if rom_loaded {
                // println!("Keys pressed: {:?}", keys_pressed);
                for _ in 0..CYCLES_PER_FRAME {
                    let redraw = self.machine.step(keys_pressed)
                        .expect(&format!("\n\nFailed to execute step {}\n\t", loop_index)); // Handle errors properly in production code

                    // if any of the cycles triggered a redraw then flag it
                    if redraw {
                        trigger_redraw = true;
                    }
                }
            }

            if trigger_redraw {
                self.gui.update_with_buffer(self.machine.frame_buffer()).expect("Failed to update GUI buffer");
                trigger_redraw = false; // Reset the flag until the next step indicates a redraw is needed
            } else {
                self.gui.update();
            }

            loop_index += 1;

        }

        Ok(())
    }
}