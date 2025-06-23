
pub static DISPLAY_WIDTH: usize = 64;
pub static DISPLAY_HEIGHT: usize = 32;
pub static MEMORY_SIZE: usize = 4096;
pub static STACK_SIZE: usize = 16; // not a spec limit
pub static VARIABLE_REGISTERS_SIZE: usize = 16;
pub static CARRY_FLAG_ADDRESS: u8 = 0xF;

pub type MemoryType = u8;
pub type StackType = u16;
pub type ProgramCounterType = u16; // these should be 12-bit addresses
pub type IndexRegisterType = u16; // these should be 12-bit addresses
pub type DelayTimerType = u8;
pub type SoundTimerType = u8;
pub type VariableRegisterType = u8;

pub static STACK_MASK: u16 = 0x0FFF; // 12-bit address mask
pub static INDEX_REGISTER_MASK: u16 = 0x0FFF; // 12-bit address mask for index register
