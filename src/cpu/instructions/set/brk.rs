use super::{aliases::StatusFlagAlias, Cpu};

impl Cpu {
    /// BRK - Force Interrupt.
    /// The BRK instruction forces the generation of an interrupt request. The program counter and
    /// processor status are pushed on the stack then the IRQ interrupt vector at $FFFE/F is loaded
    /// into the PC and the break flag in the status set to one.
    pub fn brk(&mut self) {
        // TODO: Push registers onto stack.
        // TODO: Load IRQ interrupt vector into the PC.
        self.registers.status.set_flag(StatusFlagAlias::B, true);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn b_flag_halts() {
        let mut cpu = Cpu::new();
        cpu.load_program(&[0x00]);
        cpu.run();

        assert!(cpu.registers.status.get_flag(StatusFlagAlias::B));
    }
}
