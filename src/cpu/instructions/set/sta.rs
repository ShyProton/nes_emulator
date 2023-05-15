use super::{AddressingMode, Cpu};

impl Cpu {
    /// STA - Store Accumulator.
    /// Stores the contents of the accumulator into memory.
    pub fn sta(&mut self, addr_mode: &AddressingMode) {
        let addr = self.get_operand_address(addr_mode);
        self.memory.write(addr, self.registers.accumulator);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_store_mem() {
        let mut cpu = Cpu::new();
        cpu.load_program(&[0x85, 0x69]);

        cpu.registers.accumulator = 0x55;

        cpu.run();

        assert_eq!(cpu.memory.read(0x0069), 0x55);
    }

    #[test]
    fn zerox_store_mem() {
        let mut cpu = Cpu::new();
        cpu.load_program(&[0x95, 0x60]);

        cpu.registers.index_x = 0x09;
        cpu.registers.accumulator = 0x55;

        cpu.run();

        assert_eq!(cpu.memory.read(0x0069), 0x55);
    }

    #[test]
    fn abs_store_mem() {
        let mut cpu = Cpu::new();
        cpu.load_program(&[0x8D, 0x42, 0x69]);

        cpu.registers.accumulator = 0x55;

        cpu.run();

        assert_eq!(cpu.memory.read(0x6942), 0x55);
    }

    #[test]
    fn absx_store_mem() {
        let mut cpu = Cpu::new();
        cpu.load_program(&[0x9D, 0x40, 0x69]);

        cpu.registers.index_x = 0x02;
        cpu.registers.accumulator = 0x55;

        cpu.run();

        assert_eq!(cpu.memory.read(0x6942), 0x55);
    }

    #[test]
    fn absy_store_mem() {
        let mut cpu = Cpu::new();
        cpu.load_program(&[0x99, 0x40, 0x69]);

        cpu.registers.index_y = 0x02;
        cpu.registers.accumulator = 0x55;

        cpu.run();

        assert_eq!(cpu.memory.read(0x6942), 0x55);
    }

    #[test]
    fn indx_store_mem() {
        let mut cpu = Cpu::new();
        cpu.load_program(&[0x81, 0x30]);

        cpu.registers.index_x = 0x04;
        cpu.registers.accumulator = 0x55;
        cpu.memory.write(0x0034, 0x0069);

        cpu.run();

        assert_eq!(cpu.memory.read(0x0069), 0x55);
    }

    #[test]
    fn indy_store_mem() {
        let mut cpu = Cpu::new();
        cpu.load_program(&[0x91, 0x34]);

        cpu.registers.index_y = 0x60;
        cpu.registers.accumulator = 0x55;
        cpu.memory.write(0x0034, 0x09);

        cpu.run();

        assert_eq!(cpu.memory.read(0x0069), 0x55);
    }
}
