use chip8_emu::Ram;
use chip8_emu::constants::{MEMORY_SIZE};


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ram_initialization() {
        let ram = Ram::new();
        assert_eq!(ram.len(), MEMORY_SIZE);
        for i in 0..MEMORY_SIZE {
            assert_eq!(ram.get(i).unwrap(), 0);
        }
    }

    #[test]
    fn ram_set_and_get() {
        let mut ram = Ram::new();
        ram.set(0, 42).expect("Failed to set RAM");
        let value = ram.get(0).expect("Failed to get RAM");
        assert_eq!(value, 42);
    }

    #[test]
    fn ram_reset() {
        let mut ram = Ram::new();
        ram.set(0, 42).unwrap();
        let value = ram.get(0).unwrap();
        assert_eq!(value, 42);
        ram.reset();
        let value = ram.get(0).unwrap();
        assert_eq!(value, 0);
    }
}
