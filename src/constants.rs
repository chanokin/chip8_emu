use std::collections::HashMap;
use lazy_static::lazy_static;

pub static DISPLAY_WIDTH: usize = 64;
pub static DISPLAY_HEIGHT: usize = 32;
pub static MEMORY_SIZE: usize = 4096;
pub static STACK_SIZE: usize = 16; // not a spec limit
pub static VARIABLE_REGISTERS_SIZE: usize = 16;
pub static CARRY_FLAG_ADDRESS: u8 = 15; // i.e. VARIABLE_REGISTERS_SIZE - 1
pub static NUMBER_OF_KEYS: usize = 16; // 0x0 to 0xF

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
pub static DISPLAY_SIZE: usize = DISPLAY_END_ADDRESS - DISPLAY_START_ADDRESS; // Size of the display memory
pub static FONT_START_ADDRESS: usize = 0x050; // 80 in decimal
pub static FONT_END_ADDRESS: usize = 0x0A0; // 160 in decimal
// pub static FONT_SIZE: usize = FONT_END_ADDRESS - FONT_START_ADDRESS;

pub static PROGRAM_COUNTER_START: ProgramCounterType = 0x200; // CHIP-8 programs typically start at 0x200

pub static MAX_PROGRAM_SIZE: usize = MEMORY_SIZE - PROGRAM_COUNTER_START as usize; // Maximum program size is the remaining memory after the start address

pub static UPPER_NIBBLE_MASK: u8 = 0xF0; // Mask for the upper nibble
pub static LOWER_NIBBLE_MASK: u8 = 0x0F; // Mask for the lower nibble

pub static OPCODE_MASK: u16 = 0xF000; // Mask for the opcode (upper 4 bits)



// the opcode requires a single 12-bit number from the instruction
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
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
    pub const CallSubroutine:OpCodes = OpCodes(0x2);
    pub const SkipIfEqualRegVal:OpCodes = OpCodes(0x3);
    pub const SkipIfUnequalRegVal:OpCodes = OpCodes(0x4);
    pub const SkipIfEqualRegReg:OpCodes = OpCodes(0x5);
    pub const SetRegister:OpCodes = OpCodes(0x6);
    pub const AddToRegister:OpCodes = OpCodes(0x7);
    pub const RegisterOperations:OpCodes = OpCodes(0x8);
    pub const SkipIfUnequalRegReg:OpCodes = OpCodes(0x9);
    pub const SetIndex:OpCodes = OpCodes(0xA);
    pub const JumpWithOffset:OpCodes = OpCodes(0xB);
    pub const Random:OpCodes = OpCodes(0xC);
    pub const Draw:OpCodes = OpCodes(0xD);
    pub const KeyOperations:OpCodes = OpCodes(0xE);
    pub const Miscellaneous:OpCodes = OpCodes(0xF);
}


#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct FunctionCodes(pub u8);

#[allow(non_upper_case_globals)]
impl FunctionCodes {
    pub const ClearDisplay:FunctionCodes = FunctionCodes(0xE0);
    pub const ReturnFromSubroutine:FunctionCodes = FunctionCodes(0xEE);
    /* DEBUGGING OPTION, IF MACHINE/CPU LOOPS REACH THIS WE BREAK */
    pub const GetKey:FunctionCodes = FunctionCodes(0xFB);
    pub const SkipIfKeyPressed:FunctionCodes = FunctionCodes(0x9E);
    pub const SkipIfKeyNotPressed:FunctionCodes = FunctionCodes(0xA1);
}


#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct RegisterOperations(pub u8);

#[allow(non_upper_case_globals)]
impl RegisterOperations {
    pub const SetToSecond: RegisterOperations = RegisterOperations(0x0);
    pub const OrEqual: RegisterOperations = RegisterOperations(0x1);
    pub const AndEqual: RegisterOperations = RegisterOperations(0x2);
    pub const XorEqual: RegisterOperations = RegisterOperations(0x3);
    pub const AddEqual: RegisterOperations = RegisterOperations(0x4);
    pub const SubtractEqual: RegisterOperations = RegisterOperations(0x5);
    pub const ShiftRightEqual: RegisterOperations = RegisterOperations(0x6);
    pub const SetToSecondMinusFirst: RegisterOperations = RegisterOperations(0x7);
    pub const ShiftLeftEqual: RegisterOperations = RegisterOperations(0xE);
}


#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct MiscellaneousOperations(pub u8);

#[allow(non_upper_case_globals)]
impl MiscellaneousOperations {
    pub const GetDelay: MiscellaneousOperations = MiscellaneousOperations(0x07);
    pub const GetKey: MiscellaneousOperations = MiscellaneousOperations(0x0A);
    pub const SetDelayTimer: MiscellaneousOperations = MiscellaneousOperations(0x15);
    pub const SetSoundTimer: MiscellaneousOperations = MiscellaneousOperations(0x18);
    pub const AddToIndexRegister: MiscellaneousOperations = MiscellaneousOperations(0x1E);
    pub const SetSpriteAddress: MiscellaneousOperations = MiscellaneousOperations(0x29);
    pub const StoreBCD: MiscellaneousOperations = MiscellaneousOperations(0x33);
    pub const StoreRegisters: MiscellaneousOperations = MiscellaneousOperations(0x55);
    pub const LoadRegisters: MiscellaneousOperations = MiscellaneousOperations(0x65);
}


pub static MAX_LOOP_COUNT: i64 = 1000000; // Maximum loop count to prevent infinite loops


lazy_static! {
    pub static ref MAP_KEY_STATE_TO_CHIP8: HashMap<u8, char> = HashMap::from([
        // keys 1, 2, 3, 4
        (0x0u8, 'C'),
        (0x1u8, 'D'),
        (0x2u8, 'E'),
        (0x3u8, 'F'),
        // keys Q, W, E, R
        (0x4u8, '8'),
        (0x5u8, '9'),
        (0x6u8, 'A'),
        (0x7u8, 'B'),
        // keys A, S, D, F
        (0x8u8, '4'),
        (0x9u8, '5'),
        (0xAu8, '6'),
        (0xBu8, '7'),
        // keys Z, X, C, V
        (0x8u8, '0'),
        (0x9u8, '1'),
        (0xAu8, '2'),
        (0xBu8, '3')
    ]);
}
