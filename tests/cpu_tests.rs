use chip8_emu::cpu::Cpu;
// use chip8_emu::errors::CpuError;
use chip8_emu::constants::{
    PROGRAM_COUNTER_START,
};

#[cfg(test)]
mod tests {
    use chip8_emu::constants::{DISPLAY_CELLS_PER_ROW, DISPLAY_SIZE, DISPLAY_START_ADDRESS};

    use super::*;

    #[test]
    fn test_cpu_initialization() {
        let cpu = Cpu::new();
        // Check if all registers are initialized to 0
        for i in 0..16 {
            assert_eq!(cpu.get_register_value(i).unwrap(), 0, "Register {} should be initialized to 0", i);
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
        let _ = cpu.set_program_counter(0x300); // Set program counter to a non-zero value
        cpu.set_index_register(0x400).expect("should be able to set index register"); // Set index register to a non-zero value
        cpu.push_to_stack(0x500).expect("should be able to push to stack");

        // Reset the CPU
        cpu.reset();

        // Check if registers are reset to 0
        for i in 0..16 {
            assert_eq!(cpu.get_register_value(i).unwrap(), 0, "Register {} should be reset to 0", i);
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
        assert_eq!(cpu.get_register_value(register_index).unwrap(), value, "Register {} should be set to {}", register_index, value);
    }

    #[test]
    fn test_cpu_get_register_value() {
        let mut cpu = Cpu::new();
        // Set a register to a value
        let register_index = 1;
        let value = 99;
        cpu.set_register(register_index, value).expect("should be able to set register");
        // Get the register value
        let retrieved_value = cpu.get_register_value(register_index).expect("should be able to get register");
        // Check if the retrieved value matches the set value
        assert_eq!(retrieved_value, value, "Retrieved value for register {} should be {}", register_index, value);
    }

    #[test]
    fn test_cpu_invalid_register_access() {
        let cpu = Cpu::new();
        // Attempt to get a register that is out of bounds
        let invalid_index = 16; // Register index out of bounds (Chip-8 has 16 registers)
        let result = cpu.get_register_value(invalid_index);
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

    #[test]
    fn test_cpu_index_register() {
        let mut cpu = Cpu::new();
        // Set the index register to a specific value
        let new_index = 0x400;
        let _ = cpu.set_index_register(new_index).expect("should be able to set index register");
        // Check if the index register is set correctly
        assert_eq!(cpu.get_index_register(), new_index, "Index register should be set to {}", new_index);
    }

    #[test]
    fn test_cpu_stack_operations() {
        let mut cpu = Cpu::new();
        // Push a value onto the stack
        let value_to_push = 0x500;
        cpu.push_to_stack(value_to_push).expect("should be able to push to stack");
        // Check if the stack is not empty
        assert!(!cpu.stack_is_empty(), "Stack should not be empty after pushing a value");
        // Check if the top of the stack is the pushed value
        let top_value = cpu.pop_from_stack().expect("should be able to pop from stack");
        assert_eq!(top_value, value_to_push, "Top of the stack should be {}", value_to_push);
        // Check if the stack is empty after popping
        assert!(cpu.stack_is_empty(), "Stack should be empty after popping the last value");

        assert!(cpu.pop_from_stack().is_err(), "Popping from an empty stack should return an error");
    }


    #[test]
    fn test_sub_routines_ops() {
        let mut cpu = Cpu::new();
        // Push a value onto the stack
        let value_to_push = 0x500;
        cpu.call_routine_at(value_to_push).expect("call at address");
        // Check if the stack is not empty
        assert!(!cpu.stack_is_empty(), "Stack should not be empty after pushing a value");
        // Pop the value from the stack

        assert!(cpu.return_from_subroutine().is_ok(), "should be able to return from subroutine");

        // Check if the stack is empty after popping
        assert!(cpu.stack_is_empty(), "Stack should be empty after returning from subroutine");

        // Reset CPU state
        cpu.reset();
    }

    #[test]
    fn test_draw_sprite_at() {
        let mut cpu = Cpu::new();
        let mut gpu = chip8_emu::gpu::Gpu::new();
        let mut ram = chip8_emu::ram::Ram::new();

        ram.reset();
        cpu.reset();

        // let sprite = [0xF0, 0x92, 0x88, 0x44, 0x0F]; // A simple sprite
        let sprite = [0xF0, 0x92]; // A simple sprite
        let size = sprite.len();
        let sprite_address = PROGRAM_COUNTER_START;

        ram.set_range(sprite_address, size, &sprite).expect("should be able to set sprite data");

        let from_ram = ram.get_range(sprite_address, size).expect("should be able to get sprite from ram");

        assert_eq!(from_ram, Vec::from(sprite), "RAM should contain the sprite");

        cpu.set_index_register(sprite_address as u16).expect("should be able to set index register");

        cpu.draw_sprite_at(0, 0, size as u8, &mut gpu, &mut ram).expect("should be able to draw sprite");

        let mut ram_contents = Vec::new();
        for i in 0..size {
            let block_address = DISPLAY_START_ADDRESS + i * DISPLAY_CELLS_PER_ROW; // Each byte represents 8 pixels
            let block = ram.get(block_address).expect("should be able to get display contents");
            ram_contents.push(block);
        }

        assert_eq!(ram_contents, Vec::from(sprite), "Display should contain the drawn sprite");

    }



    #[test]
    fn test_clear_display() {
        let mut cpu = Cpu::new();
        let mut gpu = chip8_emu::gpu::Gpu::new();
        let mut ram = chip8_emu::ram::Ram::new();

        ram.reset();
        cpu.reset();

        ram.set_range(DISPLAY_START_ADDRESS as u16, DISPLAY_SIZE, &[1; DISPLAY_SIZE]).expect("should be able to set display contents");

        // Clear the display
        cpu.clear_display(&mut gpu, &mut ram).expect("should be able to clear display");
        // Check if the display is cleared
        let display_contents = ram.get_range(DISPLAY_START_ADDRESS as u16, DISPLAY_SIZE).expect("should be able to get display contents");
        for &cell in display_contents.iter() {
            assert_eq!(cell, 0, "Display cell should be cleared");
        }

    }
}
