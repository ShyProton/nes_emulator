use super::{registers::Registers, Memory};

pub struct Cpu {
    pub registers: Registers,
    pub memory: Memory,
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
