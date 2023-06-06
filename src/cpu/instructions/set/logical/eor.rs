use super::{AddressingMode, Cpu, Operation};

#[cfg(test)]
use super::test_templates;

impl Cpu {
    /// EOR - Exclusive OR.
    /// An exclusive OR is performed, bit by bit, on the accumulator contents using the contents of
    /// a byte of memory.
    pub fn eor(&mut self, addr_mode: &AddressingMode) {
        self.logical(addr_mode, &Operation::Eor);
    }
}

#[cfg(test)]
mod tests {
    use super::{
        test_templates::{absolute, immediate, indirect, zero},
        Operation,
    };

    const OPERATION: Operation = Operation::Eor;

    #[test]
    fn imm_and() {
        immediate::logical(0x49, &OPERATION);
    }

    #[test]
    fn zero_and() {
        zero::logical(0x45, &OPERATION);
    }

    #[test]
    fn zero_x_and() {
        zero::x_logical(0x55, &OPERATION);
    }

    #[test]
    fn abs_and() {
        absolute::logical(0x4D, &OPERATION);
    }

    #[test]
    fn abs_x_and() {
        absolute::x_logical(0x5D, &OPERATION);
    }

    #[test]
    fn abs_y_and() {
        absolute::y_logical(0x59, &OPERATION);
    }

    #[test]
    fn ind_x_and() {
        indirect::x_logical(0x41, &OPERATION);
    }

    #[test]
    fn ind_y_and() {
        indirect::y_logical(0x51, &OPERATION);
    }
}
