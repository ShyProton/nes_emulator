use super::{AddressingMode, Cpu};

#[cfg(test)]
use super::StatusFlagAlias;

// NOTE: Since INC functions differently from INX and INY due to incrementing a location in memory
// rather than a register, the implementation and tests do not use the predefined
// register-incrementing templates used by the other two.
impl Cpu {
    /// INC - Increment Memory.
    /// Adds one to the value held at a specified memory location, setting the zero and negative
    /// flags as appropriate.
    pub fn inc(&mut self, addr_mode: &AddressingMode) {
        let addr = self.get_operand_address(addr_mode);
        let result = self.memory.read(addr).wrapping_add(1);

        self.update_zero_and_negative_flags(result);
        self.memory.write(addr, result);
    }
}

#[cfg(test)]
mod tests {
    use super::{Cpu, StatusFlagAlias};

    #[test]
    fn zero_overflow() {
        let mut cpu = Cpu::new();
        cpu.load_program(&[0xE6, 0x69]);

        cpu.memory.write(0x0069, 0xFF);

        cpu.run();

        assert_eq!(cpu.memory.read(0x0069), 0x00);
        assert!(cpu.registers.status.get_flag(StatusFlagAlias::Z));
    }

    #[test]
    fn zero_increment_mem() {
        let mut cpu = Cpu::new();
        cpu.load_program(&[0xE6, 0x69]);

        cpu.memory.write(0x0069, 0x01);

        cpu.run();

        assert_eq!(cpu.memory.read(0x0069), 0x02);
    }

    #[test]
    fn zero_x_increment_mem() {
        let mut cpu = Cpu::new();
        cpu.load_program(&[0xF6, 0x60]);

        cpu.registers.index_x = 0x09;
        cpu.memory.write(0x0069, 0x01);

        cpu.run();

        assert_eq!(cpu.memory.read(0x0069), 0x02);
    }

    #[test]
    fn abs_increment_mem() {
        let mut cpu = Cpu::new();
        cpu.load_program(&[0xEE, 0x42, 0x69]);

        cpu.memory.write(0x6942, 0x01);

        cpu.run();

        assert_eq!(cpu.memory.read(0x6942), 0x02);
    }

    #[test]
    fn abs_x_increment_mem() {
        let mut cpu = Cpu::new();
        cpu.load_program(&[0xFE, 0x40, 0x69]);

        cpu.registers.index_x = 2;
        cpu.memory.write(0x6942, 0x01);

        cpu.run();

        assert_eq!(cpu.memory.read(0x6942), 0x02);
    }
}
