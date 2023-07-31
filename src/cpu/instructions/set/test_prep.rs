use super::{aliases::RegisterAlias, AddressingMode, Cpu};

pub const ADDR: u8 = 0x0069;
pub const ABS_ADDR: u16 = 0x6942;

fn get_index_diff(mode_register: Option<&RegisterAlias>) -> u8 {
    let validate_register = |register: &RegisterAlias| match register {
        RegisterAlias::X | RegisterAlias::Y => {}
        _ => panic!("Addressing modes only support X and Y, not {register:?}"),
    };

    mode_register.map_or(0x00, |register| {
        validate_register(register);
        0x0A // This value is arbitrary.
    })
}

fn implied(cpu: &mut Cpu, opcode: u8) {
    cpu.load_program(&[opcode]);
}

fn immediate(cpu: &mut Cpu, opcode: u8, value: u8) {
    cpu.load_program(&[opcode, value]);
}

fn zero(cpu: &mut Cpu, mode_register: Option<&RegisterAlias>, opcode: u8, value: u8) {
    let diff = get_index_diff(mode_register);

    cpu.load_program(&[opcode, ADDR.wrapping_sub(diff)]);
    cpu.memory.write(ADDR.into(), value);

    if let Some(register) = mode_register {
        cpu.registers.set_register(register, diff);
    }
}

fn absolute(cpu: &mut Cpu, mode_register: Option<&RegisterAlias>, opcode: u8, value: u8) {
    let diff = get_index_diff(mode_register);

    let bytes = ABS_ADDR.to_le_bytes();

    cpu.load_program(&[opcode, bytes[0].wrapping_sub(diff), bytes[1]]);
    cpu.memory.write(ABS_ADDR, value);

    if let Some(register) = mode_register {
        cpu.registers.set_register(register, diff);
    }
}

fn indirect(cpu: &mut Cpu, opcode: u8, value: u8) {
    cpu.load_program(&[opcode, 0x34, 0x42]);

    cpu.memory.write(0x4234, ADDR);
    cpu.memory.write(ADDR.into(), value);
}

fn indirect_x(cpu: &mut Cpu, opcode: u8, value: u8) {
    cpu.load_program(&[opcode, 0x30]);
    cpu.registers.index_x = 0x04;

    cpu.memory.write(0x0034, ADDR);
    cpu.memory.write(ADDR.into(), value);
}

fn indirect_y(cpu: &mut Cpu, opcode: u8, value: u8) {
    cpu.load_program(&[opcode, 0x34]);
    cpu.registers.index_y = ADDR & 0x000F;

    cpu.memory.write(0x0034, ADDR & 0x00F0);
    cpu.memory.write(ADDR.into(), value);
}

pub fn get_addr(addr_mode: &AddressingMode) -> u16 {
    match addr_mode {
        AddressingMode::Immediate | AddressingMode::Implied => {
            panic!("{addr_mode:?} has no associated address")
        }
        AddressingMode::Absolute | AddressingMode::AbsoluteX | AddressingMode::AbsoluteY => {
            ABS_ADDR
        }
        _ => ADDR.into(),
    }
}

pub fn prepare(cpu: &mut Cpu, opcode: u8, addr_mode: &AddressingMode, value: u8) {
    match addr_mode {
        AddressingMode::Implied => implied(cpu, opcode),
        AddressingMode::Immediate => immediate(cpu, opcode, value),
        AddressingMode::ZeroPage => zero(cpu, None, opcode, value),
        AddressingMode::ZeroPageX => zero(cpu, Some(&RegisterAlias::X), opcode, value),
        AddressingMode::ZeroPageY => zero(cpu, Some(&RegisterAlias::Y), opcode, value),
        AddressingMode::Absolute => absolute(cpu, None, opcode, value),
        AddressingMode::AbsoluteX => absolute(cpu, Some(&RegisterAlias::X), opcode, value),
        AddressingMode::Indirect => indirect(cpu, opcode, value),
        AddressingMode::AbsoluteY => absolute(cpu, Some(&RegisterAlias::Y), opcode, value),
        AddressingMode::IndirectX => indirect_x(cpu, opcode, value),
        AddressingMode::IndirectY => indirect_y(cpu, opcode, value),
    };
}
