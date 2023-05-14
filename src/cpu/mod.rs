mod instructions;
mod registers;

use super::memory::Memory;
use registers::Registers;

pub struct Cpu {
    registers: Registers,
    memory: Memory,
}

impl Cpu {
    pub const fn new() -> Self {
        Self {
            registers: Registers::new(),
            memory: Memory::new(),
        }
    }

    pub fn run(&mut self) {
        // While operating, continuously find the next opcode.
        while self.execute_instruction() {}
    }

    pub fn load_program(&mut self, program: &[u8]) {
        self.memory.load(program);
        self.registers.reset(&self.memory);
    }
}
