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
                // BRK
                0x00 => {
                    return;
                }

                // LDA
                0xA9 => {
                    let param = program[self.registers.program_counter as usize];
                    self.registers.program_counter += 1;
                    self.registers.accumulator = param;

                    self.registers
                        .status
                        .set_flag('Z', self.registers.accumulator == 0);

                    self.registers
                        .status
                        .set_flag('N', self.registers.accumulator.get_nth_bit(7));
                }

                // TAX
                0xAA => {
                    self.registers.index_x = self.registers.accumulator;

                    self.registers
                        .status
                        .set_flag('Z', self.registers.index_x == 0);

                    self.registers
                        .status
                        .set_flag('N', self.registers.index_x.get_nth_bit(7));
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
    fn lda_0xa9_immediate_load_data() {
        let mut cpu = Cpu::new();
        cpu.interpret(&[0xA9, 0x05, 0x00]);

        assert_eq!(cpu.registers.accumulator, 0x05);
        assert!(!cpu.registers.status.get_flag('Z'));
        assert!(!cpu.registers.status.get_flag('N'));
    }

    #[test]
    fn lda_0xa9_zero_flag() {
        let mut cpu = Cpu::new();
        cpu.interpret(&[0xA9, 0x00, 0x00]);

        assert!(cpu.registers.status.get_flag('Z'));
    }

    #[test]
    fn tax_0xaa_move_a_to_x() {
        let mut cpu = Cpu::new();
        cpu.interpret(&[0xA9, 0x0A, 0xAA, 0x00]); // Transfer 0x0A in A to X.

        assert_eq!(cpu.registers.index_x, 0x0A);
    }
}
