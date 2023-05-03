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
                0xA9 => {
                    let param = program[self.registers.program_counter as usize];
                    self.registers.program_counter += 1;
                    self.registers.accumulator = param;

                    if self.registers.accumulator == 0 {
                        self.registers.status |= 0b0000_0010;
                    } else {
                        self.registers.status &= 0b1111_1101;
                    }

                    if self.registers.accumulator & 0b1000_0000 == 0 {
                        self.registers.status &= 0b0111_1111;
                    } else {
                        self.registers.status |= 0b1000_0000;
                    }
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
