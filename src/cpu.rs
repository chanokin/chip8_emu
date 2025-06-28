use crate::constants::{
    ProgramCounterType,
    CodeCategories,
    OpCodes,
    FunctionCodes,
    PROGRAM_COUNTER_START,
    DISPLAY_START_ADDRESS,
};
use crate::errors::CpuError;
use crate::registers::Registers;
use crate::gpu::Gpu;
use crate::ram::Ram;
use crate::stack::Stack;
pub struct Decoder;


impl Decoder {
    pub fn new() -> Self {
        Decoder
    }

    pub fn opcode(&self, instruction: u16) -> u8 {
        // Extract the opcode from the instruction
        (instruction >> 12) as u8
    }

    pub fn args(&self, instruction: u16) -> u16 {
        // Extract the arguments from the instruction
        self.nnn(instruction)
    }

    pub fn nnn(&self, instruction: u16) -> u16 {
        // Extract the 12-bit number from the instruction
        instruction & 0x0FFF
    }

    pub fn xnn(&self, instruction: u16) -> (u8, u8) {
        // Extract the 4-bit var register index/name and the 8-bit number from the instruction
        let first_register = ((instruction >> 8) & 0x0F) as u8;
        let val = (instruction & 0x00FF) as u8;

        (first_register, val)
    }

    pub fn xyn(&self, instruction: u16) -> (u8, u8, u8) {
        // Extract two 4-bit var register indices/names and a 4-bit number from the instruction
        let first_registesr = ((instruction >> 8) & 0x0F) as u8;
        let second_register = ((instruction >> 4) & 0x0F) as u8;
        let number = (instruction & 0x0F) as u8; // just a number?
        (first_registesr, second_register, number)
    }

    pub fn extraction_category(&self, opcode: u8) -> Result<CodeCategories, CpuError> {
        // Determine the category of the instruction
        match opcode {
            // 1 12-bit number (one output value)
            0x0 | 0x1 | 0x2 | 0xA | 0xB => Ok(CodeCategories::NNN),
            // 1 var register and 1 8-bit number (two output values)
            0x3 | 0x4 | 0x6 | 0x7 | 0xC => Ok(CodeCategories::XNN),
            // 2 var register and 1 4-bit number (three output values)
            0x5 | 0x8 | 0x9 | 0xD | 0xE | 0xF => Ok(CodeCategories::XYN),
            _ => Err(CpuError::InvalidOpcode(opcode)),
        }
    }
}



pub struct Cpu {
    decoder: Decoder,
    program_counter: ProgramCounterType,
    registers: Registers,
    display: Box<Option<Gpu>>,
    ram: Box<Option<Ram>>,
    stack: Box<Option<Stack>>
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            decoder: Decoder::new(),
            program_counter: PROGRAM_COUNTER_START, // CHIP-8 programs typically start at 0x200
            registers: Registers::new(),
            display: Box::new(None), // Display can be set later
            ram: Box::new(None), // RAM can be set later
            stack: Box::new(None), // Stack can be set later

        }
    }

    pub fn with_shared_objects(display: Gpu, ram: Ram, stack: Stack) -> Self {
        Cpu {
            decoder: Decoder::new(),
            program_counter: PROGRAM_COUNTER_START, // CHIP-8 programs typically start at 0x200
            registers: Registers::new(),
            display: Box::new(Some(display)),
            ram: Box::new(Some(ram)),
            stack: Box::new(Some(stack)), // Stack can be set later
        }
    }

    pub fn reset(&mut self) {
        // Reset CPU state
        self.program_counter = PROGRAM_COUNTER_START; // Reset program counter to start
    }

    pub fn decode(&self, instruction: u16) -> (u8, u16) {
        let opcode = self.decoder.opcode(instruction);
        let args = self.decoder.args(instruction);

        (opcode, args)
    }

    pub fn execute(&mut self, opcode: u8, args: u16) -> Result<(), CpuError> {
        let category = match self.decoder.extraction_category(opcode) {
            Ok(cat) => cat,
            Err(e) => return Err(e),
        };

        match category {
            CodeCategories::NNN => self.nnn(opcode, args),
            CodeCategories::XNN => self.xnn(opcode, args),
            _ => self.xyn(opcode, args),
        }
    }



    pub fn nnn(&mut self, opcode: u8, args: u16) -> Result<(), CpuError> {
        match OpCodes(opcode)  {
            OpCodes::Zero => self.zero(args),
            OpCodes::Jump => self.jump_to(args),
            OpCodes::SetIndex => self.set_index_register(args),
            _ => Err(CpuError::InvalidOpcode(opcode)),
        }

    }

    pub fn xnn(&mut self, opcode: u8, args: u16) -> Result<(), CpuError> {
        // Extract the register and number from the arguments
        let (register, number) = self.decoder.xnn(args);

        // execute the instruction based on the opcode
        match OpCodes(opcode) {
            OpCodes::SetRegister => self.set_register(register as usize, number),
            OpCodes::AddToRegister => self.add_to_register(register as usize, number),
            _ => Err(CpuError::InvalidOpcode(opcode)),
        }
    }

    pub fn xyn(&mut self, opcode: u8, args: u16) -> Result<(), CpuError> {
        // Extract the two registers and the number from the arguments
        let (register_0, register_1, number) = self.decoder.xyn(args);

        // execute the instruction based on the opcode
        match OpCodes(opcode) {
            OpCodes::Draw => self.draw_sprite_at(register_0, register_1, number),
            _ => Err(CpuError::InvalidOpcode(opcode)),
        }
    }

    fn zero(&mut self, args: u16) -> Result<(), CpuError> {
        match FunctionCodes(args) {
            FunctionCodes::ClearDisplay => self.clear_display(), // Clear the display
            FunctionCodes::ReturnFromSubroutine => self.return_from_subroutine(), // Return from subroutine (pop from stack)
            _ => self.call_routine_at(args), // Call a subroutine at the given address
        }
    }

    fn jump_to(&mut self, address: u16) -> Result<(), CpuError> {
        // Jump to the specified address
        // This would typically set the program counter to the address
        if address < PROGRAM_COUNTER_START || address >= DISPLAY_START_ADDRESS as u16 {
            return Err(CpuError::InvalidJump(address)); // Invalid address for program counter
        }

        // just set, don't increment
        self.program_counter = address;

        Ok(())
    }

    fn return_from_subroutine(&mut self) -> Result<(), CpuError> {
        Ok(())
    }

    fn call_routine_at(&mut self, address: u16) -> Result<(), CpuError> {
        Ok(())
    }

    fn clear_display(&mut self,) -> Result<(), CpuError> {
        let _ = match &mut *self.display {
            Some(display) => display.clear(),
            None => return Err(CpuError::ClearDisplay), // Display not initialized
        };
        Ok(())
    }

    fn set_register(&mut self, index: usize, value: u8) -> Result<(), CpuError> {
        match self.registers.set(index, value) {
            Ok(_) => Ok(()),
            Err(_) => Err(CpuError::SetRegisterError(index)), // translate error to CpuError
        }
    }

    fn add_to_register(&mut self, index: usize, value: u8) -> Result<(), CpuError> {
        let register_value = match self.registers.get(index) {
            Ok(val) => val,
            Err(_) => return Err(CpuError::GetRegisterError(index)),
        };

        // seems overflow is not handled in the original code
        let new_value: u16 = (register_value as u16) + (value as u16);
        let new_value: u8 = (new_value & 0xFF) as u8;

        // Set the new value in the register
        match self.registers.set(index, new_value) {
            Ok(_) => Ok(()),
            Err(_) => Err(CpuError::SetRegisterError(index)), // translate error to CpuError
        }
    }

    fn set_index_register(&mut self, address: u16) -> Result<(), CpuError> {
        // Set the index register to the specified address
        // This would typically set the index register to the address
        Ok(())
    }

    fn draw_sprite_at(&mut self, x: u8, y: u8, n: u8) -> Result<(), CpuError> {
        // Draw a sprite at the specified coordinates (x, y) with height n
        // This would typically involve accessing the display memory and drawing the sprite
        Ok(())
    }


}