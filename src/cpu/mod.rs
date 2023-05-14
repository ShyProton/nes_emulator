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
            let opcode = self.read_instruction();

            match opcode {
                0x00 => return, // BRK

                // TODO: Organize Opcodes into their own structure or enum or something
                0xA9 => self.lda(&operations::AddressingMode::Immediate),
                0xA5 => self.lda(&operations::AddressingMode::ZeroPage),

                0xA2 => self.ldx(&operations::AddressingMode::Immediate),
                0x85 => self.sta(&operations::AddressingMode::ZeroPage),
                0xAA => self.tax(),
                0xE8 => self.inx(),

                _ => panic!("{opcode:#04x} is an invalid opcode"),
            }
        }
    }

    pub fn load_program(&mut self, program: &[u8]) {
        self.memory.load(program);
        self.registers.reset(&self.memory);
    }

    fn read_instruction(&mut self) -> u8 {
        let opcode = self.memory.read(self.registers.program_counter);
        self.registers.program_counter += 1;

        opcode
    }
}
