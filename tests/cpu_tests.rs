use chip8_emu::cpu::Cpu;
use chip8_emu::errors::CpuError;
use chip8_emu::constants::{
    PROGRAM_COUNTER_START,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_reset() {
        let mut cpu = Cpu::new();
        cpu.reset();

        // Check if registers are reset to 0
        for i in 0..16 {
            assert_eq!(cpu.get_register(i).unwrap(), 0, "Register {} should be reset to 0", i);
        }

        // Check if program counter is reset
        assert_eq!(cpu.get_program_counter(), PROGRAM_COUNTER_START, "Program counter should be reset to 0x200");

        assert_eq!(cpu.get_index_register(), 0, "Index register should be reset to 0");

        // Check if stack is empty
        assert!(cpu.stack_is_empty(), "Stack should be empty after reset");
    }
}