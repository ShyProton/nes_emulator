use super::{aliases::StatusFlagAlias, AddressingMode, Cpu};

#[cfg(test)]
use super::test_prep;

#[cfg(test)]
mod tests;

#[allow(clippy::cast_sign_loss)]
pub fn u8_to_u16(byte: u8) -> u16 {
    // let padding = if byte & 0b0000_0001 << 7 == 0 {
    //     0x00
    // } else {
    //     0xFF
    // };
    //
    // u16::from_be_bytes([padding, byte])
    i16::from(byte) as u16
}

impl Cpu {
    /// Base for branching instructions.
    fn branch(&mut self, flag: StatusFlagAlias, setting: bool) {
        let addr = self.get_operand_address(&AddressingMode::Immediate);

        if self.registers.status.get_flag(flag) == setting {
            let displacement = self.memory.read(addr);

            self.registers.program_counter = self
                .registers
                .program_counter
                .wrapping_add(u8_to_u16(displacement));
        }
    }

    /// BCC - Branch if Carry Clear
    /// If the carry flag is clear then add the relative displacement to the program counter to
    /// cause a branch to a new location.
    pub fn bcc(&mut self) {
        self.branch(StatusFlagAlias::C, false);
    }

    /// BCS - Branch if Carry Set
    /// If the carry flag is set then add the relative displacement to the program counter to
    /// cause a branch to a new location.
    pub fn bcs(&mut self) {
        self.branch(StatusFlagAlias::C, true);
    }

    /// BEQ - Branch if Equal
    /// If the zero flag is set then add the relative displacement to the program counter to
    /// cause a branch to a new location.
    pub fn beq(&mut self) {
        self.branch(StatusFlagAlias::Z, true);
    }

    /// BMI - Branch if Minus
    /// If the negative flag is set then add the relative displacement to the program counter to
    /// cause a branch to a new location.
    pub fn bmi(&mut self) {
        self.branch(StatusFlagAlias::N, true);
    }

    /// BNE - Branch if Not Equal
    /// If the zero flag is clear then add the relative displacement to the program counter to
    /// cause a branch to a new location.
    pub fn bne(&mut self) {
        self.branch(StatusFlagAlias::Z, false);
    }

    /// BPL - Branch if Positive
    /// If the negative flag is clear then add the relative displacement to the program counter to
    /// cause a branch to a new location.
    pub fn bpl(&mut self) {
        self.branch(StatusFlagAlias::N, false);
    }

    /// BVC - Branch if Overflow Clear
    /// If the overflow flag is clear then add the relative displacement to the program counter to
    /// cause a branch to a new location.
    pub fn bvc(&mut self) {
        self.branch(StatusFlagAlias::V, false);
    }

    /// BVS - Branch if Overflow Set
    /// If the overflow flag is set then add the relative displacement to the program counter to
    /// cause a branch to a new location.
    pub fn bvs(&mut self) {
        self.branch(StatusFlagAlias::V, true);
    }
}
