use crate::constants::{
    CodeCategories,
};
use crate::errors::CpuError;
use crate::registers::Registers;
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
    registers: Registers,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            decoder: Decoder::new(),
            registers: Registers::new(),
        }
    }

    pub fn reset(&mut self) {
        // Reset CPU state
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
        match opcode {
            0x0 => self.zero(args),
            0x1 => self.jump_to(args),
            0xA => self.set_index_register(args),
            _ => Err(CpuError::InvalidOpcode(opcode)),
        }

    }

    pub fn xnn(&mut self, opcode: u8, args: u16) -> Result<(), CpuError> {
        let (register, number) = self.decoder.xnn(args);
        match opcode {
            0x6 => self.set_register(register as usize, number),
            0x7 => self.add_to_register(register as usize, number),
            _ => Err(CpuError::InvalidOpcode(opcode)),
        }
    }

    pub fn xyn(&mut self, opcode: u8, args: u16) -> Result<(), CpuError> {
        let (register_0, register_1, number) = self.decoder.xyn(args);
        match opcode {
            0xD => self.draw_sprite_at(register_0, register_1, number),
            _ => Err(CpuError::InvalidOpcode(opcode)),
        }
    }

    fn zero(&mut self, args: u16) -> Result<(), CpuError> {
        match args {
            0x00E0 => self.clear_display(), // Clear the display
            0x00EE => self.return_from_subroutine(), // Return from subroutine
            _ => self.call_routine_at(args), // Call a subroutine at the given address
        }
    }

    fn jump_to(&mut self, address: u16) -> Result<(), CpuError> {
        // Jump to the specified address
        // This would typically set the program counter to the address
        Ok(())
    }

    fn return_from_subroutine(&mut self) -> Result<(), CpuError> {
        Ok(())
    }

    fn call_routine_at(&mut self, address: u16) -> Result<(), CpuError> {
        Ok(())
    }

    fn clear_display(&mut self,) -> Result<(), CpuError> {
        Ok(())
    }

    fn set_register(&mut self, index: usize, value: u8) -> Result<(), CpuError> {
        if self.registers.set(index, value).is_err() {
            return Err(CpuError::RegisterOutOfBounds(index));
        }

        Ok(())
    }

    fn add_to_register(&mut self, index: usize, value: u8) -> Result<(), CpuError> {
        if let Ok(register_value) = self.registers.get(index) {
            let new_value: u16 = (register_value as u16) + (value as u16);
            if new_value > 0xFF {

            }
        }
        // Add the value to the register here
        Ok(())
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