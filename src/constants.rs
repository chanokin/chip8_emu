
pub static DISPLAY_WIDTH: usize = 64;
pub static DISPLAY_HEIGHT: usize = 32;
pub static MEMORY_SIZE: usize = 4096;
pub static STACK_SIZE: usize = 16; // not a spec limit
pub static VARIABLE_REGISTERS_SIZE: usize = 16;
pub static CARRY_FLAG_ADDRESS: u8 = 15; // i.e. VARIABLE_REGISTERS_SIZE - 1

pub type MemoryType = u8;
pub type AddressType = u16; // these should be 12-bit addresses
pub type StackType = u16;
pub type ProgramCounterType = u16; // these should be 12-bit addresses
pub type IndexRegisterType = u16; // these should be 12-bit addresses
pub type DelayTimerType = u8;
pub type SoundTimerType = u8;
pub type VariableRegisterType = u8;

pub static STACK_MASK: u16 = 0x0FFF; // 12-bit address mask
pub static INDEX_REGISTER_MASK: u16 = 0x0FFF; // 12-bit address mask for index register

pub static DISPLAY_FREQUENCY: u64 = 60; // 60Hz for the CHIP-8 emulator
pub static DISPLAY_COLS_PER_CELL: usize = 8; // Each pixel is represented by a bit in a byte
pub static DISPLAY_ROWS_PER_CELL: usize = 1; // Each pixel is represented by a bit in a byte
pub static DISPLAY_CELLS_PER_ROW: usize = DISPLAY_WIDTH / DISPLAY_COLS_PER_CELL; // Number of cells in a row
pub static DISPLAY_START_ADDRESS: usize = 0xF00; // Display memory starts at 0xF00
pub static DISPLAY_END_ADDRESS: usize = DISPLAY_START_ADDRESS + DISPLAY_HEIGHT * DISPLAY_WIDTH / DISPLAY_COLS_PER_CELL; // Display memory ends at 0xFFF

pub static FONT_START_ADDRESS: usize = 0x050; // 80 in decimal
pub static FONT_END_ADDRESS: usize = 0x0A0; // 160 in decimal
// pub static FONT_SIZE: usize = FONT_END_ADDRESS - FONT_START_ADDRESS;

pub static PROGRAM_COUNTER_START: ProgramCounterType = 0x200; // CHIP-8 programs typically start at 0x200

pub static MAX_PROGRAM_SIZE: usize = MEMORY_SIZE - PROGRAM_COUNTER_START as usize; // Maximum program size is the remaining memory after the start address

pub static UPPER_NIBBLE_MASK: u8 = 0xF0; // Mask for the upper nibble
pub static LOWER_NIBBLE_MASK: u8 = 0x0F; // Mask for the lower nibble

pub static OPCODE_MASK: u16 = 0xF000; // Mask for the opcode (upper 4 bits)



// the opcode requires a single 12-bit number from the instruction
pub enum CodeCategories {
    NNN, // 12-bit number
    XNN, // 8-bit value and a number
    XYN, // 8-bit value and a number
}

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct OpCodes(pub u8);

#[allow(non_upper_case_globals)]
impl OpCodes {
    pub const Zero:OpCodes = OpCodes(0x0);
    pub const Jump:OpCodes = OpCodes(0x1);
    pub const Subroutine:OpCodes = OpCodes(0x2);
    pub const CondRegValEq:OpCodes = OpCodes(0x3);
    pub const CondRegValNEq:OpCodes = OpCodes(0x4);
    pub const CondRegRegEq:OpCodes = OpCodes(0x5);
    pub const SetRegister:OpCodes = OpCodes(0x6);
    pub const AddToRegister:OpCodes = OpCodes(0x7);
    pub const RegisterOperations:OpCodes = OpCodes(0x8);
    pub const CondRegRegNEq:OpCodes = OpCodes(0x9);
    pub const SetIndex:OpCodes = OpCodes(0xA);
    pub const JumpWithOffset:OpCodes = OpCodes(0xB);
    pub const Random:OpCodes = OpCodes(0xC);
    pub const Draw:OpCodes = OpCodes(0xD);
    pub const KeyOperations:OpCodes = OpCodes(0xE);
    pub const MemoryOperations:OpCodes = OpCodes(0xF);
}


#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct FunctionCodes(pub u16);

#[allow(non_upper_case_globals)]
impl FunctionCodes {
    pub const ClearDisplay:FunctionCodes = FunctionCodes(0x00E0);
    pub const ReturnFromSubroutine:FunctionCodes = FunctionCodes(0x00EE);
    /* DEBUGGING OPTION, IF MACHINE/CPU LOOPS REACH THIS WE BREAK */
}

pub static MAX_LOOP_COUNT: i64 = 1000000; // Maximum loop count to prevent infinite loops
