use crate::constants::{
    MEMORY_SIZE,
    MemoryType
};

// Memory for the CHIP-8 emulator.
pub struct Ram {
    memory: [MemoryType; MEMORY_SIZE],
}