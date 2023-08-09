use super::{
    reference::{InstructionAlias, INSTRUCTION_LOOKUP},
    registers::aliases::StatusFlagAlias,
    AddressingMode, Cpu,
};

#[allow(dead_code)]
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
    /// Executes the next instruction.
    /// Returns a boolean corresponding to whether the instruction executed.
    pub fn cycle(&mut self) -> bool {
        if self.registers.status.get_flag(StatusFlagAlias::B) {
            return false;
        }

        let opcode = self.memory.read(self.registers.program_counter);

        let instruction = INSTRUCTION_LOOKUP.get(&[opcode]).map_or_else(
            || panic!("invalid opcode lookup for instruction: {opcode:0x}"),
            |instruction| instruction,
        );

        self.registers.program_counter += 1;
        self.execute_instruction(instruction);

        true
    }
}
