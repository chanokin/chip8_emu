use crate::constants::{
    VariableRegisterType,
    VARIABLE_REGISTERS_SIZE,
    CARRY_FLAG_ADDRESS,
    INDEX_REGISTER_MASK
};

pub struct Registers {
    variables: [VariableRegisterType; VARIABLE_REGISTERS_SIZE],
    index_register: u16, // 12-bit address
}