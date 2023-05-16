use super::{
    registers::{aliases::StatusFlagAlias, RegisterByte},
    AddressingMode, Cpu,
};

impl Cpu {
    pub fn update_zero_and_negative_flags(&mut self, result: u8) {
        self.registers
            .status
            .set_flag(StatusFlagAlias::Z, result == 0);
        self.registers
            .status
            .set_flag(StatusFlagAlias::N, result.get_nth_bit(7));
    }

    pub fn get_value(&mut self, addr_mode: &AddressingMode) -> u8 {
        let addr = self.get_operand_address(addr_mode);
        self.memory.read(addr)
    }
}
