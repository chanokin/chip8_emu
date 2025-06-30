use crate::ram::Ram;
use crate::cpu::Cpu;
use crate::gpu::Gpu;
use crate::constants::{
    DelayTimerType,
    SoundTimerType,

};
use crate::stack::Stack;

pub struct Machine {
    pub ram: Ram,
    pub stack: Stack,
    pub gpu: Gpu,
    pub cpu: Cpu,
    pub delay_timer: DelayTimerType,
    pub sound_timer: SoundTimerType,
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
            delay_timer: 0,
            sound_timer: 0,
        };

        machine.load_font();
        machine.reset();

        machine
    }

    pub fn reset(&mut self) {
        self.ram.reset();
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.stack.reset();
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