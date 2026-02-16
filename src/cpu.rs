use crate::constants::{
    CodeCategories, DelayTimerType, FunctionCodes, IndexRegisterType, OpCodes, ProgramCounterType,
    RegisterOperations, SoundTimerType, VariableRegisterType, MiscellaneousOperations,
    DISPLAY_START_ADDRESS, NUMBER_OF_KEYS, PROGRAM_COUNTER_START
};
use crate::errors::{
    CpuError,
};
use crate::registers::Registers;
use crate::gpu::Gpu;
use crate::ram::Ram;
use crate::stack::Stack;
pub struct Decoder;
use log::{debug, };

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
    sound_timer: SoundTimerType
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
            sound_timer: 0 // Initialize sound timer to 0
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
        debug!("\nDecoded instruction: 0x{:04X} -> Opcode: 0x{:02X}, Args: 0x{:04X}", instruction, opcode, args);
        (opcode, args)
    }

    pub fn execute(&mut self, opcode: u8, args: u16, gpu: &mut Gpu, ram: &mut Ram, key_state: [bool; 16]) -> Result<(), CpuError> {
        let category = match self.decoder.extraction_category(opcode) {
            Ok(cat) => cat,
            Err(e) => return Err(e),
        };

        debug!("\tExecuting opcode: 0x{:02X} in category: {:?}", opcode, category);

        // just trying to distribute according to how we split the instructions
        match category {
            CodeCategories::NNN => self.nnn(opcode, args, gpu, ram, key_state),
            CodeCategories::XNN => self.xnn(opcode, args, gpu, ram, key_state),
            _ => self.xyn(opcode, args, gpu, ram, key_state),
        }
    }


    pub fn update(&mut self, gpu: &mut Gpu, ram: &mut Ram, key_state: [bool; 16]) -> Result<(), CpuError> {
        // Fetch the next instruction from memory
        let instruction = self.fetch(ram)?;

        // Decode the instruction
        let (opcode, args) = self.decode(instruction);
        debug!("-> Executing instruction at PC: 0x{:04X} -> Opcode: 0x{:02X}, Args: 0x{:04X}",
                 self.program_counter - 2, opcode, args);
        // Execute the instruction
        self.execute(opcode, args, gpu, ram, key_state)?;

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

    pub fn get_register(&self, index: usize) -> Result<VariableRegisterType, CpuError> {
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
    pub fn peek_stack(&self) -> Result<ProgramCounterType, CpuError> {
        // Peek at the top value of the stack without popping it
        match self.stack.peek() {
            Ok(value) => Ok(value),
            Err(e) => Err(CpuError::StackError(e)), // Handle stack peek error
        }
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

    pub fn pop_from_stack(&mut self) -> Result<ProgramCounterType, CpuError> {
        // Pop the top value from the stack
        match self.stack.pop() {
            Ok(value) => Ok(value),
            Err(e) => Err(CpuError::StackPopError(e)), // Handle stack pop error
        }
    }

    pub fn is_key_pressed(&self, key: u8, key_state: [bool; 16]) -> bool {
        // Check if a specific key is pressed
        if key < NUMBER_OF_KEYS as u8 {
            key_state[key as usize]
        } else {
            false // Invalid key index
        }
    }

    pub fn is_key_released(&self, key: u8, key_state: [bool; 16]) -> bool {
        // Check if a specific key is released
        if key < NUMBER_OF_KEYS as u8 {
            key_state[key as usize] == false
        } else {
            false // Invalid key index
        }
    }

    // Dispatch functions based on instruction categories with a 12-bit number (NNN)
    pub fn nnn(&mut self, opcode: u8, args: u16, gpu: &mut Gpu, ram: &mut Ram, _key_state: [bool; 16]) -> Result<(), CpuError> {
        debug!("\tExecuting opcode: 0x{:02X} in category: NNN with args: 0x{:04X}", opcode, args);
        match OpCodes(opcode)  {
            OpCodes::Zero => self.zero(args, gpu, ram), // 0x0NNN
            OpCodes::Jump => self.jump_to(args), // 0x1NNN
            OpCodes::CallSubroutine => self.call_routine_at(args), // 0x2NNN
            OpCodes::SetIndex => self.set_index_register(args), // 0xANNN
            _ => Err(CpuError::InvalidOpcode(opcode)),
        }

    }

    // Dispatch functions with an 8-bit number (NN) and a register (X)
    pub fn xnn(&mut self, opcode: u8, args: u16, _gpu: &mut Gpu, ram: &mut Ram, key_state: [bool; 16]) -> Result<(), CpuError> {
        // Extract the register and number from the arguments
        let (register, number) = self.decoder.xnn(args);

        // execute the instruction based on the opcode
        match OpCodes(opcode) {
            OpCodes::SkipIfEqualRegVal => self.skip_if_equal(register as usize, number), // 0x3XNN
            OpCodes::SkipIfUnequalRegVal => self.skip_if_unequal(register as usize, number), // 0x4XNN
            OpCodes::SetRegister => self.set_register(register as usize, number), // 0x6XNN
            OpCodes::AddToRegister => self.add_to_register(register as usize, number), // 0x7XNN
            OpCodes::Random => self.random(register as usize, number), // 0xCXNN
            OpCodes::KeyOperations => self.key_operations(register as usize, number, key_state), // 0xEXNN
            OpCodes::Miscellaneous => self.miscellaneous(register as usize, number, ram, key_state), // 0xFXNN
            _ => Err(CpuError::InvalidOpcode(opcode)),
        }
    }

    // Dispatch functions with two registers (X, Y) and a number (N)
    pub fn xyn(&mut self, opcode: u8, args: u16, gpu: &mut Gpu, ram: &mut Ram, _key_state: [bool; 16]) -> Result<(), CpuError> {
        // Extract the two registers and the number from the arguments
        let (register_0, register_1, number) = self.decoder.xyn(args);

        // execute the instruction based on the opcode
        match OpCodes(opcode) {
            // 0x5XY0
            OpCodes::SkipIfEqualRegReg => self.skip_if_equal_registers(register_0 as usize, register_1 as usize), // 0x5XY0
            // 0x8XYN
            OpCodes::RegisterOperations => self.reg_ops(register_0 as usize, register_1 as usize, number, gpu, ram), // 0x8XYN
            // 0x9XY0
            OpCodes::SkipIfUnequalRegReg => self.skip_if_unequal_registers(register_0 as usize, register_1 as usize), // 0x9XY0
            // 0xDXYN
            OpCodes::Draw => self.draw_sprite_at(register_0, register_1, number, gpu, ram), // 0xDXYN
            _ => Err(CpuError::InvalidOpcode(opcode)),
        }
    }

    /* 0x0??? ---------------------------------------------------------------------- */

    pub fn zero(&mut self, args: u16, gpu: &mut Gpu, ram: &mut Ram) -> Result<(), CpuError> {
        match FunctionCodes((args & 0x00FF) as u8) {
            FunctionCodes::ClearDisplay => self.clear_display(gpu, ram), // 0x00E0
            FunctionCodes::ReturnFromSubroutine => self.return_from_subroutine(), // 0x00EE
            _ => self.call_routine_at(args), // Call a subroutine at the given address
        }
    }


    // 0x00E0
    pub fn clear_display(&mut self, gpu: &mut Gpu, ram: &mut Ram) -> Result<(), CpuError> {
        debug!("Clearing display");
        match gpu.clear(ram) {
            Ok(_) => Ok(()),
            Err(_) => Err(CpuError::ClearDisplay), // translate error to CpuError
        }
    }

    // 0x00EE
    pub fn return_from_subroutine(&mut self) -> Result<(), CpuError> {
        debug!("Returning from subroutine");
        // Return from a subroutine by popping the address from the stack
        if self.stack.is_empty() {
            return Err(CpuError::StackNotInitialized); // Handle empty stack error
        }

        let address = self.pop_from_stack()?;

        self.program_counter = address;

        Ok(())
    }


    /* 0x1??? ---------------------------------------------------------------------- */

    pub fn jump_to(&mut self, address: u16) -> Result<(), CpuError> {
        debug!("Jumping to address: 0x{:04X}", address);
        // Jump to the specified address
        // This would typically set the program counter to the address
        if address < PROGRAM_COUNTER_START || address >= DISPLAY_START_ADDRESS as u16 {
            return Err(CpuError::InvalidJump(address)); // Invalid address for program counter
        }

        // just set, don't increment
        self.program_counter = address;

        Ok(())
    }

    /* 0x2??? ---------------------------------------------------------------------- */

    pub fn call_routine_at(&mut self, address: u16) -> Result<(), CpuError> {
        debug!("Calling subroutine at address: 0x{:04X}", address);
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

    /* 0x3??? ---------------------------------------------------------------------- */
    pub fn skip_if_equal(&mut self, register: usize, value: u8) -> Result<(), CpuError> {
        debug!("Skipping next instruction if register {} equals {}", register, value);
        // Skip the next instruction if the value in the specified register equals the given value
        let register_value = match self.registers.get(register) {
            Ok(val) => val,
            Err(_) => return Err(CpuError::GetRegisterError(register)),
        };

        if register_value == value {
            self.increment_program_counter()?; // Skip next instruction
        }

        Ok(())
    }

    /* 0x4??? ---------------------------------------------------------------------- */
    pub fn skip_if_unequal(&mut self, register: usize, value: u8) -> Result<(), CpuError> {
        debug!("Skipping next instruction if register {} does not equal {}", register, value);
        // Skip the next instruction if the value in the specified register does not equal the given value
        let register_value = match self.registers.get(register) {
            Ok(val) => val,
            Err(_) => return Err(CpuError::GetRegisterError(register)),
        };

        if register_value != value {
            self.increment_program_counter()?; // Skip next instruction
        }

        Ok(())
    }

    /* 0x5??? ---------------------------------------------------------------------- */
    pub fn skip_if_equal_registers(&mut self, register_0: usize, register_1: usize) -> Result<(), CpuError> {
        debug!("Skipping next instruction if register {} equals register {}", register_0, register_1);
        // Skip the next instruction if the values in the specified registers are equal
        let value_0 = match self.registers.get(register_0) {
            Ok(val) => val,
            Err(_) => return Err(CpuError::GetRegisterError(register_0)),
        };

        let value_1 = match self.registers.get(register_1) {
            Ok(val) => val,
            Err(_) => return Err(CpuError::GetRegisterError(register_1)),
        };

        if value_0 == value_1 {
            self.increment_program_counter()?; // Skip next instruction
        }

        Ok(())
    }

    /* 0x6??? ---------------------------------------------------------------------- */
    pub fn set_register(&mut self, index: usize, value: u8) -> Result<(), CpuError> {
        debug!("Setting register {} to value {}", index, value);
        match self.registers.set(index, value) {
            Ok(_) => Ok(()),
            Err(_) => Err(CpuError::SetRegisterError(index)), // translate error to CpuError
        }
    }


    /* 0x7??? ---------------------------------------------------------------------- */
    pub fn add_to_register(&mut self, index: usize, value: u8) -> Result<(), CpuError> {
        debug!("Adding {} to register {}", value, index);

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

    /* 0x8??? ---------------------------------------------------------------------- */
    // a lot of operations are needed here
    pub fn reg_ops(&mut self, register_0: usize, register_1: usize, number: u8, _gpu: &mut Gpu, _ram: &mut Ram) -> Result<(), CpuError> {
        debug!("Performing register operation with registers {} and {}, number {}", register_0, register_1, number);
        // Perform operations based on the opcode
        let value_0 = self.get_register(register_0)?;
        let value_1 = self.get_register(register_1)?;

        // here the number is used to determine the operation
        match RegisterOperations(number) {
            RegisterOperations::SetToSecond => {
                self.set_register(register_0, value_1)?;
            },
            RegisterOperations::OrEqual => {
                let result = value_0 | value_1;
                self.set_register(register_0, result)?;
            },
            RegisterOperations::AndEqual => {
                let result = value_0 & value_1;
                self.set_register(register_0, result)?;
            },
            RegisterOperations::XorEqual => {
                let result = value_0 ^ value_1;
                self.set_register(register_0, result)?;
            },
            RegisterOperations::AddEqual => {
                let result = value_0.wrapping_add(value_1);
                self.set_register(register_0, result)?;
            },
            RegisterOperations::SubtractEqual => {
                let result = value_0.wrapping_sub(value_1);

                self.registers.set_carry_flag((value_0 >= value_1) as u8); // Set carry flag if no underflow

                self.set_register(register_0, result)?;
            },
            RegisterOperations::ShiftRightEqual => {
                let least_significant_bit = value_0 & 0x1;
                let result = value_0 >> 1;
                self.set_register(register_0, result)?;
                self.registers.set_carry_flag(least_significant_bit); // Set carry flag to least significant bit
            },
            RegisterOperations::SetToSecondMinusFirst => {
                let result = value_1.wrapping_sub(value_0);

                self.registers.set_carry_flag((value_1 >= value_0) as u8); // Set carry flag if no underflow

                self.set_register(register_0, result)?;
            },
            RegisterOperations::ShiftLeftEqual => {
                let most_significant_bit = (value_0 >> 7) & 0x1;
                let result = value_0 << 1;
                self.set_register(register_0, result)?;
                self.registers.set_carry_flag(most_significant_bit); // Set carry flag to most significant bit
            },
            _ => {return Err(CpuError::InvalidRegisterOperation(number));}, // Handle invalid operation
        }

        Ok(())
    }


    /* 0x9??? ---------------------------------------------------------------------- */

    pub fn skip_if_unequal_registers(&mut self, register_0: usize, register_1: usize) -> Result<(), CpuError> {
        debug!("Skipping next instruction if register {} does not equal register {}", register_0, register_1);
        // Skip the next instruction if the values in the specified registers are not equal
        let value_0 = match self.registers.get(register_0) {
            Ok(val) => val,
            Err(_) => return Err(CpuError::GetRegisterError(register_0)),
        };

        let value_1 = match self.registers.get(register_1) {
            Ok(val) => val,
            Err(_) => return Err(CpuError::GetRegisterError(register_1)),
        };

        if value_0 != value_1 {
            self.increment_program_counter()?; // Skip next instruction
        }

        Ok(())
    }

    /* 0xA??? ---------------------------------------------------------------------- */
    pub fn set_index_register(&mut self, address: u16) -> Result<(), CpuError> {
        debug!("Setting index register to address: 0x{:04X}", address);
        // Set the index register to the specified address
        if  address >= DISPLAY_START_ADDRESS as u16 {
            return Err(CpuError::IndexRegister(address)); // Invalid address for index register
        }

        self.index_register = (address & 0x0FFF) as IndexRegisterType; // Ensure it's a 12-bit address

        Ok(())
    }

    /* 0xB??? ---------------------------------------------------------------------- */

    pub fn jump_addres_plus_v0(&mut self, address: u16) -> Result<(), CpuError> {
        // Jump to the address specified by the offset from the index register
        let address = self.get_register(0)? as u16 + (address & 0x0FFF);

        debug!("Jumping to address with offset: 0x{:04X}", address);

        if address >= DISPLAY_START_ADDRESS as u16 {
            return Err(CpuError::InvalidJump(address)); // Invalid address for program counter
        }

        self.program_counter = address;

        Ok(())
    }

    /* 0xC??? ---------------------------------------------------------------------- */

    pub fn random(&mut self, register: usize, mask: u8) -> Result<(), CpuError> {
        // Generate a random number and store it in the specified register
        let random_value = rand::random::<u8>() & mask; // Apply the mask to the random value
        debug!("Setting register {} to random value: {}", register, random_value);
        self.set_register(register, random_value)
    }

    /* 0xD??? ---------------------------------------------------------------------- */
    pub fn draw_sprite_at(&mut self, x: u8, y: u8, sprite_height: u8, gpu: &mut Gpu, ram: &mut Ram) -> Result<(), CpuError> {
        // Draw a sprite at the specified coordinates (x, y) with height n
        // This would typically involve accessing the display memory and drawing the sprite
        let sprite_address = self.index_register;
        let x = self.get_register(x as usize)?;
        let y = self.get_register(y as usize)?;

        debug!(
            "Drawing sprite in {:04X} at coordinates ({}, {}) with height {}",
            sprite_address, x, y, sprite_height
        );

        let set_vf_true = match gpu.draw_sprite_at(x as usize, y as usize, sprite_address, sprite_height, ram) {
            Ok(val) => val,
            Err(e) => return Err(CpuError::DisplayError(e)), // Handle display error
        };

        // If the sprite was drawn and VF should be set to 1, we can set it in the registers
        self.registers.set_carry_flag(set_vf_true as u8);

        Ok(())
    }

    /* 0xE??? ---------------------------------------------------------------------- */
    pub fn key_operations(&mut self, register: usize, func_code: u8, key_state: [bool; 16]) -> Result<(), CpuError> {
        let value_0 = self.get_register(register)?;
        // Handle key operations based on the opcode
        match FunctionCodes(func_code) {
            FunctionCodes::SkipIfKeyPressed => self.skip_if_key_pressed(value_0, key_state), // 0xEX9E
            FunctionCodes::SkipIfKeyNotPressed => self.skip_if_key_not_pressed(value_0, key_state), // 0xEXA1
            _ => Err(CpuError::InvalidOpcode(func_code)), // Handle invalid opcode
        }
    }

    pub fn skip_if_key_pressed(&mut self, key_id: VariableRegisterType, key_state: [bool; 16]) -> Result<(), CpuError> {
        debug!("Skipping next instruction if key {} is pressed", key_id);
        // Skip the next instruction if the key corresponding to the value in the specified register is pressed
        if self.is_key_pressed(key_id, key_state) {
            self.increment_program_counter()?; // Skip next instruction
        }

        Ok(())
    }

    pub fn skip_if_key_not_pressed(&mut self, key_id: VariableRegisterType, key_state: [bool; 16]) -> Result<(), CpuError> {
        debug!("Skipping next instruction if key {} is not pressed", key_id);
        // Skip the next instruction if the key corresponding to the value in the specified register is not pressed
        if self.is_key_released(key_id, key_state) {
            self.increment_program_counter()?; // Skip next instruction
        }

        Ok(())
    }


    /* 0xF??? ---------------------------------------------------------------------- */

    pub fn miscellaneous(&mut self, register: usize, func_code: u8, _ram: &mut Ram, key_state: [bool; 16]) -> Result<(), CpuError> {
        // Handle miscellaneous operations based on the opcode
        match MiscellaneousOperations(func_code) {
            MiscellaneousOperations::GetDelay => self.set_reg_to_delay(register), // 0xFX07
            MiscellaneousOperations::GetKey => self.set_reg_to_key(register, key_state), // 0xFX0A
            MiscellaneousOperations::SetDelayTimer => self.set_delay_to_reg(register), // 0xFX15
            MiscellaneousOperations::SetSoundTimer => self.set_sound_to_reg(register), // 0xFX18
            MiscellaneousOperations::AddToIndexRegister => self.add_index_to_reg(register), // 0xFX1E
            MiscellaneousOperations::SetSpriteAddress => self.set_index_reg_to_char(register), // 0xFX29
            _ => Err(CpuError::InvalidOpcode(func_code)), // Handle invalid opcode
        }
    }

    pub fn set_reg_to_delay(&mut self, register: usize) -> Result<(), CpuError> {
        let delay_value = self.delay_timer;
        debug!("Setting register {} to delay timer value: {}", register, delay_value);
        self.set_register(register, delay_value)
    }

    pub fn set_reg_to_key(&mut self, register: usize, _key_state: [bool; 16]) -> Result<(), CpuError> {
        // block until a key is pressed and set the register to the key value
        // todo:
        debug!("Setting register {} to key value: {}", register, 0);
        self.set_register(register,0)
    }

    pub fn set_delay_to_reg(&mut self, register: usize) -> Result<(), CpuError> {
        let value = self.get_register(register)?;
        debug!("Setting delay timer to register {} value: {}", register, value);
        self.set_delay_timer(value as DelayTimerType);
        Ok(())
    }

    pub fn set_sound_to_reg(&mut self, register: usize) -> Result<(), CpuError> {
        let value = self.get_register(register)?;
        debug!("Setting sound timer to register {} value: {}", register, value);
        self.set_sound_timer(value as SoundTimerType);
        Ok(())
    }

    pub fn add_index_to_reg(&mut self, register: usize) -> Result<(), CpuError> {
        let value = self.get_register(register)?;
        debug!("Adding index register value {} to register {}", self.index_register, register);
        let new_value = self.index_register.wrapping_add(value as IndexRegisterType); // Ensure it's a 12-bit value
        self.set_index_register(new_value)?;
        Ok(())
    }

    pub fn set_index_reg_to_char(&mut self, _register: usize) -> Result<(), CpuError> {
        // Set the index register to the address of the character corresponding to the value in the specified register
        // todo!("Implement set_index_reg_to_char");
        // let value = self.get_register(register)?;
        // debug!("Setting index register to character address for register {} value: {}", register, value);
        // if value > 0xF {
        //     return Err(CpuError::InvalidCharacterRegister(value)); // Handle invalid character register
        // }

        // self.index_register = (value as IndexRegisterType) * 5; // Each character is 5 bytes

        Ok(())
    }




}