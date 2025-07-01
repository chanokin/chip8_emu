use crate::ram::Ram;
use crate::cpu::Cpu;
use crate::gpu::Gpu;
use crate::stack::Stack;
use crate::constants::{MAX_LOOP_COUNT, MAX_PROGRAM_SIZE, PROGRAM_COUNTER_START};
use crate::errors::MachineError;

pub struct Machine {
    pub ram: Ram,
    pub stack: Stack,
    pub gpu: Gpu,
    pub cpu: Cpu,
    pub kill_switch: bool,
}

impl Machine {
    pub fn new() -> Self {
        let ram = Ram::new();
        let stack = Stack::new();
        let gpu = Gpu::new();
        let cpu = Cpu::new();
        let mut machine = Machine {
            ram,
            stack,
            gpu,
            cpu,
            kill_switch: false,
        };

        machine.reset();
        machine.load_font();

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
        // self.ram.load_font();
    }

    pub fn load_program(&mut self, program: &[u8]) -> Result<(), MachineError> {

        if program.len() > MAX_PROGRAM_SIZE {
            return Err(MachineError::ProgramTooLarge(program.len(), MAX_PROGRAM_SIZE));
        }

        match self.ram.set_range(PROGRAM_COUNTER_START, program.len(), program) {
            Ok(_) => {},
            Err(e) => {
                return Err(MachineError::MemoryError(e));
            }
        };

        self.reset();

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), String> {
        let mut count: i64 = 0;

        loop {
            match self.cpu.update(&mut self.gpu, &mut self.ram) {
                Ok(_) => {
                    // Continue running the CPU
                }
                Err(e) => {
                    eprintln!("Error: {:?}", e);
                    self.kill_switch = true;
                }
            };
            if self.kill_switch {
                break;
            }

            count += 1;
            if count == MAX_LOOP_COUNT {
                eprintln!("Maximum loop count reached, stopping execution to prevent infinite loop.");
                self.kill_switch = true;
            }
        }
        Ok(())
    }

}