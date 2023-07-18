use super::{aliases::RegisterAlias, Cpu};

fn create_index_diff(cpu: &mut Cpu, mode_register: Option<&RegisterAlias>) -> u8 {
    let validate_register = |register: &RegisterAlias| match register {
        RegisterAlias::X | RegisterAlias::Y => {}
        _ => panic!("Addressing modes only support X and Y, not {register:?}"),
    };

    match mode_register {
        Some(register) => {
            validate_register(register);
            cpu.registers.set_register(register, 0x0A);
            0x0A // This value is arbitrary.
        }
        None => 0x00,
    }
}

pub fn implied(cpu: &mut Cpu, opcode: u8) {
    cpu.load_program(&[opcode]);
}

pub fn immediate(cpu: &mut Cpu, opcode: u8, value: u8) {
    cpu.load_program(&[opcode, value]);
}

pub fn zero(cpu: &mut Cpu, mode_register: Option<&RegisterAlias>, opcode: u8, value: u8) {
    let diff = create_index_diff(cpu, mode_register);

    cpu.load_program(&[opcode, 0x69_u8.wrapping_sub(diff)]);
    cpu.memory.write(0x0069, value);
}

pub fn absolute(cpu: &mut Cpu, mode_register: Option<&RegisterAlias>, opcode: u8, value: u8) {
    let diff = create_index_diff(cpu, mode_register);

    cpu.load_program(&[opcode, 0x42_u8.wrapping_sub(diff), 0x69]);
    cpu.memory.write(0x6942, value);
}
