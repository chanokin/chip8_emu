use crate::constants::{
    StackType,
    STACK_SIZE,
    STACK_MASK,
};

use crate::errors::StackError;

pub struct Stack {
    pub stack: [StackType; STACK_SIZE],
    pub stack_pointer: usize, // where in the stack we are currently pointing
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            stack: [0; STACK_SIZE],
            stack_pointer: 0,
        }
    }

    pub fn push(&mut self, value: StackType) -> Result<usize, StackError> {
        if self.stack_pointer >= STACK_SIZE {
            return Err(StackError::Overflow);
        }

        self.stack[self.stack_pointer] = value & STACK_MASK;
        self.stack_pointer += 1;

        Ok(self.stack_pointer)
    }

    pub fn pop(&mut self) -> Result<StackType, StackError> {
        if self.stack_pointer == 0 {
            return Err(StackError::Underflow);
        }


        self.stack_pointer -= 1;

        Ok(self.stack[self.stack_pointer] & STACK_MASK)

    }

    pub fn is_empty(&self) -> bool {
        self.stack_pointer == 0
    }

    pub fn is_full(&self) -> bool {
        self.stack_pointer == STACK_SIZE
    }

    pub fn len(&self) -> usize {
        self.stack_pointer
    }

    pub fn peek(&self) -> Result<StackType, StackError> {
        if self.stack_pointer == 0 {
            return Err(StackError::Underflow);
        }

        Ok(self.stack[self.stack_pointer - 1] & STACK_MASK)
    }

    pub fn clear(&mut self) {
        self.stack = [0; STACK_SIZE];
        self.stack_pointer = 0;
    }

}