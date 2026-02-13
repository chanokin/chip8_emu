use crate::constants::{
    MemoryType, MEMORY_SIZE, PROGRAM_COUNTER_START
};

use crate::errors::MemoryError;

// Memory for the CHIP-8 emulator.
pub struct Ram {
    pub memory: [MemoryType; MEMORY_SIZE],
}


impl Ram {
    pub fn new() -> Self {
        Ram {
            memory: [0; MEMORY_SIZE],
        }
    }

    pub fn reset(&mut self) {
        self.memory.fill(0);
    }

    pub fn get(&self, address: usize) -> Result<MemoryType, MemoryError> {
        if address >= MEMORY_SIZE {
            return Err(MemoryError::OutOfBounds);
        }
        Ok(self.memory[address])
    }

    pub fn get_range(&self, address: u16, size: usize) -> Result<Vec<MemoryType>, MemoryError> {
        let address = address as usize;
        if address >= MEMORY_SIZE {
            return Err(MemoryError::OutOfBounds);
        }

        if address + size > MEMORY_SIZE {
            return Err(MemoryError::OutOfBounds);
        }

        Ok(self.memory[address..address + size].to_vec())
    }

    pub fn set_range_font(&mut self, address: u16, size: usize, values: &[u8]) -> Result<(), MemoryError> {
        let address = address as usize;
        if address >= MEMORY_SIZE {
            return Err(MemoryError::OutOfBounds);
        }

        if address + size > MEMORY_SIZE {
            return Err(MemoryError::OutOfBounds);
        }

        for (i, &value) in values.iter().enumerate() {

            if i >= size {
                println!("Warning: Attempting to write beyond the specified size. Stopping write operation.");
                break; // Prevent out of bounds write
            }

            self.memory[address + i] = value;
        }

        Ok(())
    }

    pub fn set_range(&mut self, address: u16, size: usize, values: &[u8]) -> Result<(), MemoryError> {
        if address < PROGRAM_COUNTER_START {
            return Err(MemoryError::OutOfBounds);
        }

        let address = address as usize;
        if address >= MEMORY_SIZE {
            return Err(MemoryError::OutOfBounds);
        }

        if address + size > MEMORY_SIZE {
            return Err(MemoryError::OutOfBounds);
        }

        for (i, &value) in values.iter().enumerate() {

            if i >= size {
                println!("Warning: Attempting to write beyond the specified size. Stopping write operation.");
                break; // Prevent out of bounds write
            }

            self.memory[address + i] = value;
        }

        Ok(())
    }

    pub fn set(&mut self, address: usize, value: MemoryType) -> Result<(), MemoryError> {
        if address >= MEMORY_SIZE {
            return Err(MemoryError::OutOfBounds);
        }
        self.memory[address] = value;
        Ok(())
    }

    pub fn len(&self) -> usize {
        MEMORY_SIZE
    }

    pub fn is_empty(&self) -> bool {
        self.memory.iter().all(|&x| x == 0)
    }
}
