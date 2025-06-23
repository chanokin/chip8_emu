use crate::constants::{
    StackType,
    STACK_SIZE,
    STACK_MASK,
};

pub struct Stack {
    stack: [StackType; STACK_SIZE],
    sp: usize, // stack pointer
}