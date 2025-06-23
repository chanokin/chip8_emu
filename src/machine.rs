use crate::ram::Ram;
use crate::constants::{
    ProgramCounterType,
    IndexRegisterType,
    DelayTimerType,
    SoundTimerType,
};
use crate::stack::Stack;

pub struct Machine {
    pub ram: Ram,
    pub program_counter: ProgramCounterType,
    pub index_register: IndexRegisterType,
    pub delay_timer: DelayTimerType,
    pub sound_timer: SoundTimerType,
    pub stack: Stack,
}