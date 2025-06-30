use crate::constants::{
    MEMORY_SIZE,
    MemoryType
};

use crate::errors::MemoryError;

// Memory for the CHIP-8 emulator.
pub struct Ram {
    memory: [MemoryType; MEMORY_SIZE],
}


impl Ram {
    pub fn new() -> Self {
        Ram {
            memory: [0; MEMORY_SIZE],
        }
    }

    pub fn reset(&mut self) {
        self.memory = [0; MEMORY_SIZE];
    }

    pub fn get(&self, address: usize) -> Result<MemoryType, MemoryError> {
        if address >= MEMORY_SIZE {
            return Err(MemoryError::OutOfBounds);
        }
        Ok(self.memory[address])
    }

    pub fn get_range(&self, address: usize, size: usize) -> Result<Vec<MemoryType>, MemoryError> {
        if address >= MEMORY_SIZE {
            return Err(MemoryError::OutOfBounds);
        }

        if address + size > MEMORY_SIZE {
            return Err(MemoryError::OutOfBounds);
        }

        Ok(self.memory[address..address + size].to_vec())
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
}
