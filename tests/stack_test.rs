use chip8_emu::Stack;
use chip8_emu::constants::{STACK_MASK};

#[cfg(test)]
mod tests {
    use super::*;
    use chip8_emu::constants::{STACK_SIZE, StackType};

    #[test]
    fn stack_initialization() {
        let stack = Stack::new();
        assert_eq!(stack.stack.len(), STACK_SIZE);
        assert_eq!(stack.stack_pointer, 0);
    }

    #[test]
    fn stack_push() {
        let mut stack = Stack::new();
        let _ = stack.push(0x1234);
        assert_eq!(stack.stack[0], 0x1234 & STACK_MASK);
        assert_eq!(stack.stack_pointer, 1);
    }

    #[test]
    fn stack_pop() {
        let mut stack = Stack::new();
        let _ = stack.push(0x1234);
        let value = stack.pop().unwrap();
        assert_eq!(value, 0x1234 & STACK_MASK);
        assert_eq!(stack.stack_pointer, 0);
    }

    #[test]
    fn stack_pop_empty() {
        let mut stack = Stack::new();
        assert!(stack.pop().is_err()); // This should panic
    }

    #[test]
    fn stack_push_overflow() {
        let mut stack = Stack::new();
        for i in 0..STACK_SIZE {
            let _ = stack.push(i as StackType);
        }
        assert!(stack.is_full());
        assert!(stack.push(0x1234).is_err()); // This should return an Err
    }

    #[test]
    fn stack_peek() {
        let mut stack = Stack::new();
        let _ = stack.push(0x1234);
        let value = stack.peek().unwrap();
        assert_eq!(value, 0x1234 & STACK_MASK);
        assert_eq!(stack.stack_pointer, 1);
    }

    #[test]
    fn stack_peek_empty() {
        let stack = Stack::new();
        assert!(stack.peek().is_err()); // This should return an Err
    }

    #[test]
    fn stack_is_empty() {
        let stack = Stack::new();
        assert!(stack.is_empty());
    }

    #[test]
    fn stack_is_full() {
        let mut stack = Stack::new();
        for i in 0..STACK_SIZE {
            let _ = stack.push(i as StackType);
        }
        assert!(stack.is_full());
    }
}

