use crate::constants::{
    VariableRegisterType,
    VARIABLE_REGISTERS_SIZE,
    CARRY_FLAG_ADDRESS,
};
use crate::errors::MemoryError;

pub struct Registers {
    pub variables: [VariableRegisterType; VARIABLE_REGISTERS_SIZE],
}


impl Registers {
    pub fn new() -> Self {
        Self {
            variables: [0; VARIABLE_REGISTERS_SIZE],
        }
    }

    pub fn reset(&mut self) {
        self.variables = [0; VARIABLE_REGISTERS_SIZE];
    }

    pub fn get(&self, index: usize) -> Result<VariableRegisterType, MemoryError> {

        if index >= VARIABLE_REGISTERS_SIZE {
            return Err(MemoryError::OutOfBounds);
        }

        Ok(self.variables[index])
    }

    pub fn set(&mut self, index: usize, value: VariableRegisterType) -> Result<(), MemoryError> {
        if index >= VARIABLE_REGISTERS_SIZE {
            return Err(MemoryError::OutOfBounds);
        }

        self.variables[index] = value;
        Ok(())
    }

    pub fn carry_flag(&self) -> VariableRegisterType {
        self.variables[CARRY_FLAG_ADDRESS as usize]
    }

    pub fn set_carry_flag(&mut self, value: VariableRegisterType) {
        self.variables[CARRY_FLAG_ADDRESS as usize] = value & 1; // Ensure it's either 0 or 1
    }

    pub fn flip_carry_flag(&mut self) {
        self.variables[CARRY_FLAG_ADDRESS as usize] ^= 1;
    }



}