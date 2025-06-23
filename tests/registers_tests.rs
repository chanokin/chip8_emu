use chip8_emu::Registers;
use chip8_emu::constants::{VARIABLE_REGISTERS_SIZE};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registers_initialization() {
        let registers = Registers::new();
        assert_eq!(registers.variables.len(), VARIABLE_REGISTERS_SIZE);
        for &value in registers.variables.iter() {
            assert_eq!(value, 0);
        }
    }

    #[test]
    fn registers_set_and_get() {
        let mut registers = Registers::new();
        registers.set(0, 42).expect("Failed to set register");
        let value = registers.get(0).expect("Failed to get register");
        assert_eq!(value, 42);
    }

    #[test]
    fn registers_reset() {
        let mut registers = Registers::new();
        registers.set(0, 42).unwrap();
        let value = registers.get(0).unwrap();
        assert_eq!(value, 42);

        registers.reset();

        let value = registers.get(0).unwrap();
        assert_eq!(value, 0);
    }

    #[test]
    fn registers_out_of_bounds() {
        let mut registers = Registers::new();
        assert!(registers.set(VARIABLE_REGISTERS_SIZE, 42).is_err());
        assert!(registers.get(VARIABLE_REGISTERS_SIZE).is_err());
    }


}