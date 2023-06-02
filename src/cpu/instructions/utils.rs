use super::{
    registers::{aliases::StatusFlagAlias, RegisterByte},
    Cpu,
};

impl Cpu {
    pub fn update_zero_and_negative_flags(&mut self, result: u8) {
        self.registers
            .status
            .set_flag(StatusFlagAlias::Z, result == 0)
            .set_flag(StatusFlagAlias::N, result.get_nth_bit(7));
    }
}
