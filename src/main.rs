

pub use chip8_emu::Machine;

fn main() {
    println!("Hello, world!");

    let mut machine = Machine::new();
    machine.reset();
}
