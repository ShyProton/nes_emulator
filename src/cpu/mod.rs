mod registers;

use registers::{Register, Registers};

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
                0xA9 => {
                    let param = program[self.registers.program_counter as usize];
                    self.registers.program_counter += 1;
                    self.registers.accumulator = param;

                    self.registers
                        .status
                        .set('Z', self.registers.accumulator == 0);

                    self.registers
                        .status
                        .set('N', self.registers.accumulator.get_nth_bit(7));
                }

                _ => todo!(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let cpu = Cpu::new();
    }
}
