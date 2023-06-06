use super::{AddressingMode, Cpu, LogicalOperation};

#[cfg(test)]
use super::test_templates;

impl Cpu {
    /// ORA - Logical Inclusive OR.
    /// An inclusive OR is performed, bit by bit, on the accumulator contents using the contents of
    /// a byte of memory.
    pub fn ora(&mut self, addr_mode: &AddressingMode) {
        self.logical(addr_mode, &LogicalOperation::Ora);
    }
}

#[cfg(test)]
mod tests {
    use super::{
        test_templates::{absolute, immediate, indirect, zero},
        LogicalOperation,
    };

    const OPERATION: LogicalOperation = LogicalOperation::Ora;

    #[test]
    fn imm_and() {
        immediate::logical(0x09, &OPERATION);
    }

    #[test]
    fn zero_and() {
        zero::logical(0x05, &OPERATION);
    }

    #[test]
    fn zero_x_and() {
        zero::x_logical(0x15, &OPERATION);
    }

    #[test]
    fn abs_and() {
        absolute::logical(0x0D, &OPERATION);
    }

    #[test]
    fn abs_x_and() {
        absolute::x_logical(0x1D, &OPERATION);
    }

    #[test]
    fn abs_y_and() {
        absolute::y_logical(0x19, &OPERATION);
    }

    #[test]
    fn ind_x_and() {
        indirect::x_logical(0x01, &OPERATION);
    }

    #[test]
    fn ind_y_and() {
        indirect::y_logical(0x11, &OPERATION);
    }
}
