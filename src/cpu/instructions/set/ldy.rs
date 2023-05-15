use super::{AddressingMode, Cpu};

impl Cpu {
    /// LDY - Load Y Register.
    /// Loads a byte of memory into the Y register setting the zero and negative flags as
    /// appropriate.
    pub fn ldy(&mut self, addr_mode: &AddressingMode) {
        let value = self.get_value(addr_mode);

        self.registers.index_y = value;
        self.update_zero_and_negative_flags(self.registers.index_y);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn imm_load_data() {
        let mut cpu = Cpu::new();
        cpu.load_program(&[0xA0, 0x05]);
        cpu.run();

        assert_eq!(cpu.registers.index_y, 0x05);
        assert!(!cpu.registers.status.get_flag('Z'));
        assert!(!cpu.registers.status.get_flag('N'));
    }

    #[test]
    fn imm_zero_flag() {
        let mut cpu = Cpu::new();
        cpu.load_program(&[0xA0, 0x00]);
        cpu.run();

        assert!(cpu.registers.status.get_flag('Z'));
    }

    #[test]
    fn zero_from_memory() {
        let mut cpu = Cpu::new();
        cpu.load_program(&[0xA4, 0x10, 0x00]);

        cpu.memory.write(0x0010, 0x55);

        cpu.run();

        assert_eq!(cpu.registers.index_y, 0x55);
    }

    #[test]
    fn zerox_from_memory() {
        let mut cpu = Cpu::new();
        cpu.load_program(&[0xB4, 0x06]);

        cpu.memory.write(0x0010, 0x55);
        cpu.registers.index_x = 0x0A;

        cpu.run();

        assert_eq!(cpu.registers.index_y, 0x55);
    }

    #[test]
    fn abs_from_memory() {
        let mut cpu = Cpu::new();
        cpu.load_program(&[0xAC, 0x34, 0x12]);

        cpu.memory.write(0x1234, 0x55);

        cpu.run();

        assert_eq!(cpu.registers.index_y, 0x55);
    }

    #[test]
    fn absx_from_memory() {
        let mut cpu = Cpu::new();
        cpu.load_program(&[0xBC, 0x00, 0x12]);

        cpu.memory.write(0x1234, 0x55);
        cpu.registers.index_x = 0x34;

        cpu.run();

        assert_eq!(cpu.registers.index_y, 0x55);
    }
}
