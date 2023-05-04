mod operations;
mod registers;

use registers::Registers;

pub struct Cpu {
    registers: Registers,
}

impl Cpu {
    pub const fn new() -> Self {
        Self {
            registers: Registers::new(),
        }
    }

    pub fn interpret(&mut self, program: &[u8]) {
        self.registers.reset_registers();

        loop {
            let opcode = program[self.registers.program_counter as usize];
            self.registers.program_counter += 1;

            match opcode {
                0x00 => return, // BRK

                0xA9 => self.lda(program),
                0xAA => self.tax(),

                _ => panic!("Invalid opcode"),
            }
        }
    }
}
