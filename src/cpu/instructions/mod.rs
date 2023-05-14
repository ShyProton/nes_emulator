mod addressing_mode;
mod instruction;
mod set;

use super::{registers::Register, Cpu};
use addressing_mode::AddressingMode;

impl Cpu {
    fn update_zero_and_negative_flags(&mut self, result: u8) {
        self.registers.status.set_flag('Z', result == 0);
        self.registers.status.set_flag('N', result.get_nth_bit(7));
    }

    fn get_value(&mut self, addr_mode: &AddressingMode) -> u8 {
        let addr = self.get_operand_address(addr_mode);
        self.memory.read(addr)
    }
}
