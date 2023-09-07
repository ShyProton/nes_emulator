use super::{
    registers::{aliases::StatusFlagAlias, Registers},
    Memory,
};

#[derive(PartialEq, Eq)]
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
        while !self.registers.status.get_flag(StatusFlagAlias::B) {
            self.cycle();
        }
    }

    pub fn load_program(&mut self, program: &[u8]) {
        self.memory.load(program);
        self.registers.reset(&self.memory);
    }
}
