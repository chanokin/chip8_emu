use std::fmt::Display;

#[derive(Debug)]
pub enum StackError {
    Underflow,
    Overflow,
}

#[derive(Debug)]
pub enum MemoryError {
    OutOfBounds,
}

#[derive(Debug)]
pub enum DisplayError {
    OutOfBounds,
    CoordinateError(usize, usize),
}

impl Display for DisplayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DisplayError::OutOfBounds => write!(f, "Display coordinates are out of bounds"),
            DisplayError::CoordinateError(x, y) => write!(f, "Invalid coordinates: ({}, {})", x, y),
        }
    }
}


#[derive(Debug)]
pub enum CpuError {
    InvalidOpcode(u8),
    RegisterOutOfBounds(usize),
}

impl Display for CpuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CpuError::InvalidOpcode(opcode) => write!(f, "Invalid opcode: {:#04x}", opcode),
            CpuError::RegisterOutOfBounds(index) => write!(f, "Register index out of bounds: {}", index),
        }
    }
}