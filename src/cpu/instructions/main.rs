use super::{AddressingMode, Cpu, INSTRUCTION_LOOKUP};

pub struct Instruction {
    name: &'static str,
    bytes: u8,
    cycles: u8,
    addr_mode: AddressingMode,
}

impl Instruction {
    pub const fn new(name: &'static str, bytes: u8, cycles: u8, addr_mode: AddressingMode) -> Self {
        Self {
            name,
            bytes,
            cycles,
            addr_mode,
        }
    }
}

impl Cpu {
    /// Executes the next CPU instruction as defined in the memory.
    /// Only returns false if the instruction is to halt the program.
    pub fn execute_instruction(&mut self) -> bool {
        let opcode = self.memory.read(self.registers.program_counter);

        let instruction = INSTRUCTION_LOOKUP.get(&[opcode]).map_or_else(
            || panic!("invalid opcode lookup for instruction"),
            |instruction| instruction,
        );

        self.registers.program_counter += 1;

        match instruction.name {
            "BRK" => return false,

            "LDA" => self.lda(&instruction.addr_mode),
            "LDX" => self.ldx(&instruction.addr_mode),
            "LDY" => self.ldy(&instruction.addr_mode),

            "STA" => self.sta(&instruction.addr_mode),
            "STX" => self.stx(&instruction.addr_mode),
            "STY" => self.sty(&instruction.addr_mode),

            "TAX" => self.tax(),
            "INX" => self.inx(),
            _ => panic!("{} is not a valid instruction", instruction.name),
        }

        true
    }
}
