
pub static DISPLAY_WIDTH: usize = 64;
pub static DISPLAY_HEIGHT: usize = 32;
pub static MEMORY_SIZE: usize = 4096;
pub static STACK_SIZE: usize = 16; // not a spec limit
pub static VARIABLE_REGISTERS_SIZE: usize = 16;
pub static CARRY_FLAG_ADDRESS: u8 = 15; // i.e. VARIABLE_REGISTERS_SIZE - 1

pub type MemoryType = u8;
pub type StackType = u16;
pub type ProgramCounterType = u16; // these should be 12-bit addresses
pub type IndexRegisterType = u16; // these should be 12-bit addresses
pub type DelayTimerType = u8;
pub type SoundTimerType = u8;
pub type VariableRegisterType = u8;

pub static STACK_MASK: u16 = 0x0FFF; // 12-bit address mask
pub static INDEX_REGISTER_MASK: u16 = 0x0FFF; // 12-bit address mask for index register

pub static DISPLAY_FREQUENCY: u64 = 60; // 60Hz for the CHIP-8 emulator
pub static DISPLAY_START_ADDRESS: usize = 0xF00; // Display memory starts at 0xF00
pub static DISPLAY_END_ADDRESS: usize = 0xFFF; // Display memory ends at 0xFFF

pub static FONT_START_ADDRESS: usize = 0x050; // 80 in decimal
pub static FONT_END_ADDRESS: usize = 0x0A0; // 160 in decimal
// pub static FONT_SIZE: usize = FONT_END_ADDRESS - FONT_START_ADDRESS;

pub static PROGRAM_COUNTER_START: ProgramCounterType = 0x200; // CHIP-8 programs typically start at 0x200

pub static UPPER_NIBBLE_MASK: u8 = 0xF0; // Mask for the upper nibble
pub static LOWER_NIBBLE_MASK: u8 = 0x0F; // Mask for the lower nibble

pub static OPCODE_MASK: u16 = 0xF000; // Mask for the opcode (upper 4 bits)

// the opcode requires a single 12-bit number from the instruction
pub enum CodeCategories {
    NNN, // 12-bit number
    XNN, // 8-bit value and a number
    XYN, // 8-bit value and a number
}
