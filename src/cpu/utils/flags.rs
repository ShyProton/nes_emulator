use super::{Cpu, StatusFlagAlias};

impl Cpu {
    pub fn update_zero_and_negative_flags(&mut self, result: u8) {
        self.registers
            .status
            .set_flag(StatusFlagAlias::Z, result == 0)
            .set_flag(StatusFlagAlias::N, result & (0b0000_0001 << 7) != 0);
    }
}
