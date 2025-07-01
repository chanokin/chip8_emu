use std::fmt::Display;

#[derive(Debug)]
pub enum StackError {
    Underflow,
    Overflow,
    NotInitialized,
}

#[derive(Debug)]
pub enum MemoryError {
    OutOfBounds,
    NotInitialized,
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
    MemoryError(MemoryError),
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
            DisplayError::MemoryError(e) => write!(f, "Memory error: {:?}", e),
        }
    }
}


#[derive(Debug)]
pub enum CpuError {
    InvalidOpcode(u8),
    RegisterOutOfBounds(usize),
    ClearDisplay,
    GetRegisterError(usize),
    SetRegisterError(usize),
    InvalidJump(u16),
    IndexRegister(u16),
    StackPushError(StackError),
    StackPopError(StackError),
    StackNotInitialized,
    DisplayError(DisplayError),
    MemoryError(MemoryError),
    DelayTimerUnderflow,
    SoundTimerUnderflow,
    ProgramCounterOverflow,
    InvalidProgramCounter(u16),
}

impl Display for CpuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CpuError::InvalidOpcode(opcode) => write!(f, "Invalid opcode: {:#04x}", opcode),
            CpuError::RegisterOutOfBounds(index) => write!(f, "Register index out of bounds: {}", index),
            CpuError::ClearDisplay => write!(f, "Failed to clear display"),
            CpuError::GetRegisterError(index) => write!(f, "Failed to get register at index: {}", index),
            CpuError::SetRegisterError(index) => write!(f, "Failed to set register at index: {}", index),
            CpuError::InvalidJump(address) => write!(f, "Invalid jump address: {:#04x}", address),
            CpuError::StackPushError(error) => write!(f, "Stack push error: {:?}", error),
            CpuError::StackPopError(error) => write!(f, "Stack pop error: {:?}", error),
            CpuError::StackNotInitialized => write!(f, "Stack is not initialized"),
            CpuError::DisplayError(error) => write!(f, "Display error: {}", error),
            CpuError::MemoryError(error) => write!(f, "Memory error: {:?}", error),
            CpuError::IndexRegister(address) => {
                write!(f, "Index register error with address: {:#04x}", address)
            },
            CpuError::DelayTimerUnderflow => write!(f, "Delay timer underflow"),
            CpuError::SoundTimerUnderflow => write!(f, "Sound timer underflow"),
            CpuError::ProgramCounterOverflow => write!(f, "Program counter overflow"),
            CpuError::InvalidProgramCounter(address) => {
                write!(f, "Invalid program counter value: {:#04x}", address)
            },

        }
    }
}

#[derive(Debug)]
pub enum MachineError {
    CpuError(CpuError),
    MemoryError(MemoryError),
    StackError(StackError),
    DisplayError(DisplayError),
}

impl Display for MachineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MachineError::CpuError(e) => write!(f, "CPU error: {}", e),
            MachineError::MemoryError(e) => write!(f, "Memory error: {:?}", e),
            MachineError::StackError(e) => write!(f, "Stack error: {:?}", e),
            MachineError::DisplayError(e) => write!(f, "Display error: {}", e),
        }
    }
}