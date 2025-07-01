use chip8_emu::cpu::Cpu;
// use chip8_emu::errors::CpuError;
use chip8_emu::constants::{
    PROGRAM_COUNTER_START,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_initialization() {
        let cpu = Cpu::new();
        // Check if all registers are initialized to 0
        for i in 0..16 {
            assert_eq!(cpu.get_register(i).unwrap(), 0, "Register {} should be initialized to 0", i);
        }
        // Check if program counter is initialized to 0x200
        assert_eq!(cpu.get_program_counter(), PROGRAM_COUNTER_START, "Program counter should be initialized to 0x200");
        // Check if index register is initialized to 0
        assert_eq!(cpu.get_index_register(), 0, "Index register should be initialized to 0");
        // Check if stack is empty
        assert!(cpu.stack_is_empty(), "Stack should be empty on initialization");
    }

    #[test]
    fn test_cpu_reset() {
        let mut cpu = Cpu::new();
        cpu.set_register(0, 5).expect("should be able to set register"); // Set a register to a non-zero value
        cpu.set_program_counter(0x300); // Set program counter to a non-zero value
        cpu.set_index_register(0x400).expect("should be able to set index register"); // Set index register to a non-zero value
        cpu.push_to_stack(0x500).expect("should be able to push to stack");

        // Reset the CPU
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

    #[test]
    fn test_cpu_set_register() {
        let mut cpu = Cpu::new();
        // Set a register to a value
        let register_index = 0;
        let value = 42;
        cpu.set_register(register_index, value).expect("should be able to set register");
        // Check if the register value is set correctly
        assert_eq!(cpu.get_register(register_index).unwrap(), value, "Register {} should be set to {}", register_index, value);
    }

    #[test]
    fn test_cpu_get_register() {
        let mut cpu = Cpu::new();
        // Set a register to a value
        let register_index = 1;
        let value = 99;
        cpu.set_register(register_index, value).expect("should be able to set register");
        // Get the register value
        let retrieved_value = cpu.get_register(register_index).expect("should be able to get register");
        // Check if the retrieved value matches the set value
        assert_eq!(retrieved_value, value, "Retrieved value for register {} should be {}", register_index, value);
    }

    #[test]
    fn test_cpu_invalid_register_access() {
        let cpu = Cpu::new();
        // Attempt to get a register that is out of bounds
        let invalid_index = 16; // Register index out of bounds (Chip-8 has 16 registers)
        let result = cpu.get_register(invalid_index);
        // Check if the error is returned correctly
        assert!(result.is_err(), "Getting register {} should return an error", invalid_index);
    }

    #[test]
    fn test_cpu_program_counter() {
        let mut cpu = Cpu::new();
        // Set the program counter to a specific value
        let new_address = 0x300;
        let _ = cpu.set_program_counter(new_address);
        // Check if the program counter is set correctly
        assert_eq!(cpu.get_program_counter(), new_address, "Program counter should be set to {}", new_address);
    }
}