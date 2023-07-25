use super::{registers::aliases::StatusFlagAlias, Cpu};

impl Cpu {
    pub fn get_stack_addr(&self) -> u16 {
        0x0100 + u16::from(self.registers.stack_pointer)
    }

    pub fn update_zero_and_negative_flags(&mut self, result: u8) {
        self.registers
            .status
            .set_flag(StatusFlagAlias::Z, result == 0)
            .set_flag(StatusFlagAlias::N, result & (0b0000_0001 << 7) != 0);
    }
}
