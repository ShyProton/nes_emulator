use super::{AddressingMode, ArithmeticMode, Cpu};

#[cfg(test)]
use super::test_templates;

impl Cpu {
    pub fn adc(&mut self, addr_mode: &AddressingMode) {
        self.arithmetic(addr_mode, &ArithmeticMode::Addition);
    }
}

#[cfg(test)]
mod tests {
    use super::{
        test_templates::{absolute, immediate, indirect, zero},
        ArithmeticMode,
    };

    const ARITHMETIC_MODE: ArithmeticMode = ArithmeticMode::Addition;

    #[test]
    fn imm_add() {
        immediate::arithmetic(0x69, &ARITHMETIC_MODE);
    }

    #[test]
    fn zero_add() {
        zero::arithmetic(0x65, &ARITHMETIC_MODE);
    }

    #[test]
    fn zero_x_add() {
        zero::x_arithmetic(0x75, &ARITHMETIC_MODE);
    }

    #[test]
    fn abs_add() {
        absolute::arithmetic(0x6D, &ARITHMETIC_MODE);
    }

    #[test]
    fn abs_x_add() {
        absolute::x_arithmetic(0x7D, &ARITHMETIC_MODE);
    }

    #[test]
    fn abs_y_add() {
        absolute::y_arithmetic(0x79, &ARITHMETIC_MODE);
    }

    #[test]
    fn ind_x_add() {
        indirect::x_arithmetic(0x61, &ARITHMETIC_MODE);
    }

    #[test]
    fn ind_y_add() {
        indirect::y_arithmetic(0x71, &ARITHMETIC_MODE);
    }
}
