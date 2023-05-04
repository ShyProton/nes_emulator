use cpu::Cpu;

mod cpu;

fn main() {
    let mut cpu = Cpu::new();
    cpu.interpret(&[0xA9, 0x0A, 0xAA, 0x00]); // Transfer 0x0A in A to X.
}
