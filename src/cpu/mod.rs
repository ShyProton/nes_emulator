mod operations;
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
        loop {
            let opcode = self.memory.read(self.registers.program_counter);
            self.registers.program_counter += 1;

            match opcode {
                0x00 => return, // BRK

                0xA9 => self.lda(),
                0xA2 => self.ldx(),
                0xAA => self.tax(),
                0xE8 => self.inx(),

                _ => panic!("Invalid opcode"),
            }
        }
    }

    pub fn load_program(&mut self, program: &[u8]) {
        self.memory.load(program);
        self.registers.reset_registers();
        self.registers.program_counter = Memory::PRG_ROM_START;
    }
}
