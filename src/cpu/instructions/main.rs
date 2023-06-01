use super::{
    reference::{InstructionAlias, INSTRUCTION_LOOKUP},
    AddressingMode, Cpu,
};

pub struct Instruction {
    pub alias: InstructionAlias,
    bytes: u8,
    cycles: u8,
    pub addr_mode: AddressingMode,
}

impl Instruction {
    pub const fn new(
        alias: InstructionAlias,
        bytes: u8,
        cycles: u8,
        addr_mode: AddressingMode,
    ) -> Self {
        Self {
            alias,
            bytes,
            cycles,
            addr_mode,
        }
    }
}

impl Cpu {
    pub fn cycle(&mut self) {
        let opcode = self.memory.read(self.registers.program_counter);

        let instruction = INSTRUCTION_LOOKUP.get(&[opcode]).map_or_else(
            || panic!("invalid opcode lookup for instruction"),
            |instruction| instruction,
        );

        self.registers.program_counter += 1;

        self.execute_instruction(instruction);
    }
}
