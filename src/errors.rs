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
    SetPixelError(usize, usize),
    GetPixelError(usize, usize),
    FlipPixelError(usize, usize),
    MemoryNotInitialized,
    ClearFailed,
}

impl Display for DisplayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DisplayError::OutOfBounds => write!(f, "Display coordinates are out of bounds"),
            DisplayError::CoordinateError(x, y) => write!(f, "Invalid coordinates: ({}, {})", x, y),
            DisplayError::MemoryNotInitialized => write!(f, "Display memory is not initialized"),
            DisplayError::ClearFailed => write!(f, "Failed to clear display memory"),
            DisplayError::SetPixelError(x, y) => write!(f, "Failed to set pixel at ({}, {})", x, y),
            DisplayError::GetPixelError(x, y) => write!(f, "Failed to get pixel at ({}, {})", x, y),
            DisplayError::FlipPixelError(x, y) => write!(f, "Failed to flip pixel at ({}, {})", x, y),
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