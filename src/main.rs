use cpu::Cpu;
use memory::Memory;

mod cpu;
mod memory;

fn main() {
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xA9, 0x0A, 0xAA, 0x00]); // Transfer 0x0A in A to X.
    cpu.run();
}
