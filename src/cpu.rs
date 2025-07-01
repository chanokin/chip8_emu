use crate::constants::{
    ProgramCounterType,
    IndexRegisterType,
    CodeCategories,
    OpCodes,
    FunctionCodes,
    DelayTimerType,
    SoundTimerType,
    PROGRAM_COUNTER_START,
    DISPLAY_START_ADDRESS,
};
use crate::errors::{
    CpuError,
};
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
    registers: Registers,
    stack: Stack,
    program_counter: ProgramCounterType,
    index_register: IndexRegisterType,
    delay_timer: DelayTimerType,
    sound_timer: SoundTimerType,

}

impl Cpu {
    pub fn new() -> Self {
        let mut cpu = Cpu {
            decoder: Decoder::new(),
            registers: Registers::new(),
            stack: Stack::new(), // Stack can be set later
            program_counter: PROGRAM_COUNTER_START, // CHIP-8 programs typically start at 0x200
            index_register: 0, // Initialize index register to 0
            delay_timer: 0, // Initialize delay timer to 0
            sound_timer: 0, // Initialize sound timer to 0
        };

        cpu.reset(); // Reset CPU state to ensure everything is initialized correctly

        cpu
    }

    pub fn fetch(&mut self, ram: &Ram) -> Result<u16, CpuError> {
        // Fetch the next instruction from memory
        let instruction_msb = match ram.get(self.program_counter as usize) {
            Ok(value) => value,
            Err(e) => return Err(CpuError::MemoryError(e)), // Handle memory error
        };

        let instruction_lsb = match ram.get((self.program_counter + 1) as usize) {
            Ok(value) => value,
            Err(e) => return Err(CpuError::MemoryError(e)), // Handle memory error
        };

        let instruction = ((instruction_msb as u16) << 8) | (instruction_lsb as u16);

        self.increment_program_counter()?;

        Ok(instruction)
    }

    pub fn update(&mut self, gpu: &mut Gpu, ram: &mut Ram) -> Result<(), CpuError> {
        // Fetch the next instruction from memory
        let instruction = self.fetch(ram)?;

        // Decode the instruction
        let (opcode, args) = self.decode(instruction);

        // Execute the instruction
        self.execute(opcode, args, gpu, ram)?;

        // Increment the program counter

        Ok(())
    }

    pub fn set_program_counter(&mut self, value: ProgramCounterType) -> Result<(), CpuError> {
        // Set the program counter to a specific value
        if value < PROGRAM_COUNTER_START || value >= DISPLAY_START_ADDRESS as u16 {
            return Err(CpuError::InvalidProgramCounter(value)); // Handle invalid program counter value
        }

        self.program_counter = value;
        Ok(())
    }

    pub fn increment_program_counter(&mut self) -> Result<(), CpuError> {
        // Increment the program counter by 2 (as CHIP-8 instructions are 2 bytes long)
        if self.program_counter + 2 >= DISPLAY_START_ADDRESS as u16 {
            return Err(CpuError::ProgramCounterOverflow); // Handle program counter overflow
        }

        self.program_counter += 2;

        Ok(())
    }

    pub fn get_program_counter(&self) -> ProgramCounterType {
        self.program_counter
    }

    pub fn set_delay_timer(&mut self, value: DelayTimerType) {
        // Set the delay timer to a specific value
        self.delay_timer = value;
    }

    pub fn decrease_delay_timer(&mut self) -> Result<(), CpuError> {
        // Decrease the delay timer by 1
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
            Ok(())
        } else {
            Err(CpuError::DelayTimerUnderflow) // Handle underflow error
        }
    }

    pub fn get_delay_timer(&self) -> DelayTimerType {
        self.delay_timer
    }

    pub fn set_sound_timer(&mut self, value: SoundTimerType) {
        // Set the sound timer to a specific value
        self.sound_timer = value;
    }

    pub fn decrease_sound_timer(&mut self) -> Result<(), CpuError> {
        // Decrease the sound timer by 1
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
            Ok(())
        } else {
            Err(CpuError::SoundTimerUnderflow) // Handle underflow error
        }
    }

    pub fn get_register(&self, index: usize) -> Result<u8, CpuError> {
        match self.registers.get(index) {
            Ok(value) => Ok(value),
            Err(_) => Err(CpuError::GetRegisterError(index)), // Handle out of bounds error
        }
    }

    pub fn get_index_register(&self) -> IndexRegisterType {
        self.index_register
    }

    pub fn get_stack(&self) -> &Stack {
        &self.stack
    }

    pub fn stack_is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    pub fn push_to_stack(&mut self, value: ProgramCounterType) -> Result<(), CpuError> {
        // Push the current program counter onto the stack
        if let Err(e) = self.stack.push(value) {
            return Err(CpuError::StackPushError(e)); // Handle stack push error
        }

        Ok(())
    }

    pub fn reset(&mut self) {
        // Reset CPU state
        self.program_counter = PROGRAM_COUNTER_START; // Reset program counter to start
        self.stack.reset(); // Clear the stack
        self.registers.reset(); // Reset all registers to 0
        self.index_register = 0; // Reset index register to 0
        self.delay_timer = 0; // Reset delay timer to 0
        self.sound_timer = 0; // Reset sound timer to 0
    }

    pub fn decode(&self, instruction: u16) -> (u8, u16) {
        let opcode = self.decoder.opcode(instruction);
        let args = self.decoder.args(instruction);

        (opcode, args)
    }

    pub fn execute(&mut self, opcode: u8, args: u16, gpu: &mut Gpu, ram: &mut Ram) -> Result<(), CpuError> {
        let category = match self.decoder.extraction_category(opcode) {
            Ok(cat) => cat,
            Err(e) => return Err(e),
        };

        match category {
            CodeCategories::NNN => self.nnn(opcode, args, gpu, ram),
            CodeCategories::XNN => self.xnn(opcode, args, gpu, ram),
            _ => self.xyn(opcode, args, gpu, ram),
        }
    }


    pub fn nnn(&mut self, opcode: u8, args: u16, gpu: &mut Gpu, ram: &mut Ram) -> Result<(), CpuError> {
        match OpCodes(opcode)  {
            OpCodes::Zero => self.zero(args, gpu, ram),
            OpCodes::Jump => self.jump_to(args),
            OpCodes::SetIndex => self.set_index_register(args),
            _ => Err(CpuError::InvalidOpcode(opcode)),
        }

    }

    pub fn xnn(&mut self, opcode: u8, args: u16, _gpu: &mut Gpu, ram: &mut Ram) -> Result<(), CpuError> {
        // Extract the register and number from the arguments
        let (register, number) = self.decoder.xnn(args);

        // execute the instruction based on the opcode
        match OpCodes(opcode) {
            OpCodes::SetRegister => self.set_register(register as usize, number),
            OpCodes::AddToRegister => self.add_to_register(register as usize, number),
            _ => Err(CpuError::InvalidOpcode(opcode)),
        }
    }

    pub fn xyn(&mut self, opcode: u8, args: u16, gpu: &mut Gpu, ram: &mut Ram) -> Result<(), CpuError> {
        // Extract the two registers and the number from the arguments
        let (register_0, register_1, number) = self.decoder.xyn(args);

        // execute the instruction based on the opcode
        match OpCodes(opcode) {
            OpCodes::Draw => self.draw_sprite_at(register_0, register_1, number, gpu, ram),
            _ => Err(CpuError::InvalidOpcode(opcode)),
        }
    }

    pub fn zero(&mut self, args: u16, gpu: &mut Gpu, ram: &mut Ram) -> Result<(), CpuError> {
        match FunctionCodes(args) {
            FunctionCodes::ClearDisplay => self.clear_display(gpu, ram), // Clear the display
            FunctionCodes::ReturnFromSubroutine => self.return_from_subroutine(), // Return from subroutine (pop from stack)
            _ => self.call_routine_at(args), // Call a subroutine at the given address
        }
    }

    pub fn jump_to(&mut self, address: u16) -> Result<(), CpuError> {
        // Jump to the specified address
        // This would typically set the program counter to the address
        if address < PROGRAM_COUNTER_START || address >= DISPLAY_START_ADDRESS as u16 {
            return Err(CpuError::InvalidJump(address)); // Invalid address for program counter
        }

        // just set, don't increment
        self.program_counter = address;

        Ok(())
    }

    pub fn return_from_subroutine(&mut self) -> Result<(), CpuError> {
        // Return from a subroutine by popping the address from the stack
        if self.stack.is_empty() {
            return Err(CpuError::StackNotInitialized); // Handle empty stack error
        }

        match self.stack.pop() {
            Ok(_) => Ok(()),
            Err(e) => Err(CpuError::StackPopError(e)), // Handle stack pop error
        }
    }

    pub fn call_routine_at(&mut self, address: u16) -> Result<(), CpuError> {
        // Set the program counter to the address
        if address < PROGRAM_COUNTER_START || address >= DISPLAY_START_ADDRESS as u16 {
            return Err(CpuError::InvalidJump(address)); // Invalid address for program counter
        }

        if let Err(e) = self.stack.push(self.program_counter) {
            return Err(CpuError::StackPushError(e)); // Handle stack push error
        }

        self.program_counter = address;

        Ok(())
    }

    pub fn clear_display(&mut self, gpu: &mut Gpu, ram: &mut Ram) -> Result<(), CpuError> {
        match gpu.clear(ram) {
            Ok(_) => Ok(()),
            Err(_) => Err(CpuError::ClearDisplay), // translate error to CpuError
        }
    }

    pub fn set_register(&mut self, index: usize, value: u8) -> Result<(), CpuError> {
        match self.registers.set(index, value) {
            Ok(_) => Ok(()),
            Err(_) => Err(CpuError::SetRegisterError(index)), // translate error to CpuError
        }
    }

    pub fn add_to_register(&mut self, index: usize, value: u8) -> Result<(), CpuError> {
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

    pub fn set_index_register(&mut self, address: u16) -> Result<(), CpuError> {
        // Set the index register to the specified address
        if  address >= DISPLAY_START_ADDRESS as u16 {
            return Err(CpuError::IndexRegister(address)); // Invalid address for index register
        }

        self.index_register = (address & 0x0FFF) as IndexRegisterType; // Ensure it's a 12-bit address

        Ok(())
    }

    pub fn draw_sprite_at(&mut self, x: u8, y: u8, sprite_height: u8, gpu: &mut Gpu, ram: &mut Ram) -> Result<(), CpuError> {
        // Draw a sprite at the specified coordinates (x, y) with height n
        // This would typically involve accessing the display memory and drawing the sprite
        let sprite_address = self.index_register;

        let set_vf_true = match gpu.draw_sprite_at(x as usize, y as usize, sprite_address, sprite_height, ram) {
            Ok(val) => val,
            Err(e) => return Err(CpuError::DisplayError(e)), // Handle display error
        };

        // If the sprite was drawn and VF should be set to 1, we can set it in the registers
        self.registers.set_carry_flag(set_vf_true as u8);

        Ok(())
    }


}