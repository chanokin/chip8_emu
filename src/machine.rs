use crate::ram::Ram;
use crate::cpu::Cpu;
use crate::gpu::Gpu;
use crate::stack::Stack;
use crate::buzzer::Buzzer;
use crate::constants::{FONT_START_ADDRESS, MAX_LOOP_COUNT, MAX_PROGRAM_SIZE, PROGRAM_COUNTER_START};
use crate::errors::MachineError;
use crate::font::FONT;
use log::{debug, };

pub struct Machine {
    pub ram: Ram,
    pub stack: Stack,
    pub gpu: Gpu,
    pub cpu: Cpu,
    pub buzzer: Buzzer,
    pub kill_switch: bool,
}

impl Machine {
    pub fn new(buzzer_volume: f32, buzzer_tone_frequency: f32) -> Self {
        let ram = Ram::new();
        let stack = Stack::new();
        let gpu = Gpu::new();
        let cpu = Cpu::new();
        let buzzer = Buzzer::new(buzzer_volume, buzzer_tone_frequency);

        let mut machine = Machine {
            ram,
            stack,
            gpu,
            cpu,
            buzzer,
            kill_switch: false,
        };

        machine.reset();

        // todo: should we load the font here?
        // machine.load_font();

        machine
    }

    pub fn reset(&mut self) {
        self.ram.reset();
        self.stack.reset();
        let _ = self.gpu.reset(&mut self.ram);
        self.cpu.reset();
        self.kill_switch = false;
    }

    pub fn load_font(&mut self) {
        let flat_font = FONT.as_flattened();
      for (i, &byte) in flat_font.iter().enumerate() {
            let address = FONT_START_ADDRESS + i;
            if let Err(e) = self.ram.set(address, byte) {
                eprintln!("Error loading font at address {}: {:?}", address, e);
            }
        }
    }

    pub fn load_program(&mut self, program: &[u8]) -> Result<(), MachineError> {

        if program.len() > MAX_PROGRAM_SIZE {
            return Err(MachineError::ProgramTooLarge(program.len(), MAX_PROGRAM_SIZE));
        }

        self.reset();

        match self.ram.set_range(PROGRAM_COUNTER_START, program.len(), program) {
            Ok(_) => {},
            Err(e) => {
                return Err(MachineError::MemoryError(e));
            }
        };


        Ok(())
    }

    pub fn step(&mut self, key_state: [bool; 16]) -> Result<bool, MachineError> {
            let cpu: &mut Cpu = &mut self.cpu;
            let gpu: &mut Gpu = &mut self.gpu;
            let ram: &mut Ram = &mut self.ram;

            let trigger_redraw = match cpu.update(gpu, ram, key_state) {
                Ok(trigger) => trigger,
                Err(e) => return Err(MachineError::CpuError(e)),
            };

            if cpu.get_sound_timer() > 0 {
                debug!("Sound timer: {}", cpu.get_sound_timer());
                self.buzzer.turn_on();

            } else {
                self.buzzer.turn_off();
            }

            Ok(trigger_redraw)

    }

    pub fn run(&mut self, max_loops:Option<i64>, key_state: [bool; 16]) -> Result<(), MachineError> {
        let max_loops = match max_loops {
            Some(value) if value > 0 => value,
            _ => MAX_LOOP_COUNT, // Default to MAX_LOOP_COUNT if no valid max_loops is provided
        };
        let mut loop_count: i64 = 0;

        loop {
            if self.kill_switch {
                break;
            }

            self.step(key_state)?;

            loop_count += 1;
            if loop_count == max_loops {
                eprintln!("Maximum loop count reached, stopping execution to prevent infinite loop.");
                self.kill_switch = true;
            }
        }
        Ok(())
    }

    pub fn frame_buffer(&self) -> Vec<u32> {
        self.gpu.buffer_as_u8_matrix(&self.ram)
                .unwrap()
                .iter()
                .flatten()
                .map(|&pixel| if pixel > 0 { 0xFFFFFF } else { 0x000000 })
                .collect()
        // self.gpu.buffer_as_u8()
        //     .iter()
        //     .map(|&pixel| if pixel > 0 { 0xFFFFFF } else { 0x000000 })
        //     .collect()
    }

}