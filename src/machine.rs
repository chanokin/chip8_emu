use crate::ram::Ram;
use crate::cpu::Cpu;
use crate::constants::{
    ProgramCounterType,
    IndexRegisterType,
    DelayTimerType,
    SoundTimerType,
    PROGRAM_COUNTER_START,
};
use crate::stack::Stack;

pub struct Machine {
    pub ram: Ram,
    pub cpu: Cpu,
    pub program_counter: ProgramCounterType,
    pub index_register: IndexRegisterType,
    pub delay_timer: DelayTimerType,
    pub sound_timer: SoundTimerType,
    pub stack: Stack,
}

impl Machine {
    pub fn new() -> Self {
        let mut machine = Machine {
            ram: Ram::new(),
            cpu: Cpu::new(),
            program_counter: PROGRAM_COUNTER_START, // CHIP-8 programs typically start at 0x200
            index_register: 0,
            delay_timer: 0,
            sound_timer: 0,
            stack: Stack::new(),
        };

        machine.load_font();
        machine.link_ram_to_cpu();
        machine.link_ram_to_display();
        machine.reset();
        machine
    }

    pub fn reset(&mut self) {
        self.ram.reset();
        self.program_counter = PROGRAM_COUNTER_START;
        self.index_register = 0;
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.stack = Stack::new();
    }

    pub fn load_font(&mut self) {
        // self.ram.load_font();
    }

    pub fn link_ram_to_cpu(&mut self) {
        // self.cpu.ram = &mut self.ram;
    }

    pub fn link_ram_to_display(&mut self) {
        // self.display.ram = &mut self.ram;
    }
}