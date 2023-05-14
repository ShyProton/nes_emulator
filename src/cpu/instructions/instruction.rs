use super::{AddressingMode, Cpu};

use phf::phf_map;

pub struct Instruction {
    name: &'static str,
    bytes: u8,
    cycles: u8,
    addr_mode: AddressingMode,
}

impl Instruction {
    const fn new(name: &'static str, bytes: u8, cycles: u8, addr_mode: AddressingMode) -> Self {
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

        let instruction = CPU_INSTRUCTIONS.get(&[opcode]).map_or_else(
            || panic!("invalid opcode lookup for instruction"),
            |instruction| instruction,
        );

        self.registers.program_counter += 1;

        match instruction.name {
            "BRK" => return false,
            "LDA" => self.lda(&instruction.addr_mode),
            "LDX" => self.ldx(&instruction.addr_mode),
            "STA" => self.sta(&instruction.addr_mode),
            "TAX" => self.tax(),
            "INX" => self.inx(),
            _ => panic!("{} is not a valid instruction", instruction.name),
        }

        true
    }
}

// TODO: Implement the rest of the CPU instructions.
pub static CPU_INSTRUCTIONS: phf::Map<[u8; 1], Instruction> = phf_map! {
    [0x00] => Instruction::new("BRK", 1, 7, AddressingMode::Implied),

    [0xA9] => Instruction::new("LDA", 2, 2, AddressingMode::Immediate),
    [0xA5] => Instruction::new("LDA", 2, 3, AddressingMode::ZeroPage),

    [0xA2] => Instruction::new("LDX", 2, 2, AddressingMode::Immediate),

    [0x85] => Instruction::new("STA", 2, 3, AddressingMode::ZeroPage),

    [0xAA] => Instruction::new("TAX", 1, 2, AddressingMode::Implied),
    [0xE8] => Instruction::new("INX", 1, 2, AddressingMode::Implied)
};
