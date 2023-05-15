use super::Cpu;

pub mod imm {
    use super::Cpu;

    pub fn load_data(opcode: u8, target_alias: char) {
        let mut cpu = Cpu::new();
        cpu.load_program(&[opcode, 0x55]);
        cpu.run();

        assert_eq!(*cpu.registers.by_alias(target_alias), 0x55);
        assert!(!cpu.registers.status.get_flag('Z'));
        assert!(!cpu.registers.status.get_flag('N'));
    }

    pub fn z_flag_set(opcode: u8) {
        let mut cpu = Cpu::new();
        cpu.load_program(&[opcode, 0x00]);
        cpu.run();

        assert!(cpu.registers.status.get_flag('Z'));
    }
}

pub mod zero {
    use super::Cpu;

    pub fn from_memory(opcode: u8, target_alias: char) {
        let mut cpu = Cpu::new();
        cpu.load_program(&[opcode, 0x10]);

        cpu.memory.write(0x0010, 0x55);

        cpu.run();

        assert_eq!(*cpu.registers.by_alias(target_alias), 0x55);
    }

    pub fn x_from_memory(opcode: u8, target_alias: char) {
        let mut cpu = Cpu::new();
        cpu.load_program(&[opcode, 0x06]);

        cpu.memory.write(0x0010, 0x55);
        cpu.registers.index_x = 0x0A;

        cpu.run();

        assert_eq!(*cpu.registers.by_alias(target_alias), 0x55);
    }

    pub fn y_from_memory(opcode: u8, target_alias: char) {
        let mut cpu = Cpu::new();
        cpu.load_program(&[opcode, 0x06]);

        cpu.memory.write(0x0010, 0x55);
        cpu.registers.index_y = 0x0A;

        cpu.run();

        assert_eq!(*cpu.registers.by_alias(target_alias), 0x55);
    }
}

pub mod abs {
    use super::Cpu;

    pub fn from_memory(opcode: u8, target_alias: char) {
        let mut cpu = Cpu::new();
        cpu.load_program(&[opcode, 0x34, 0x12]);

        cpu.memory.write(0x1234, 0x55);

        cpu.run();

        assert_eq!(*cpu.registers.by_alias(target_alias), 0x55);
    }

    pub fn x_from_memory(opcode: u8, target_alias: char) {
        let mut cpu = Cpu::new();
        cpu.load_program(&[opcode, 0x00, 0x12]);

        cpu.memory.write(0x1234, 0x55);
        cpu.registers.index_x = 0x34;

        cpu.run();

        assert_eq!(*cpu.registers.by_alias(target_alias), 0x55);
    }

    pub fn y_from_memory(opcode: u8, target_alias: char) {
        let mut cpu = Cpu::new();
        cpu.load_program(&[opcode, 0x00, 0x12]);

        cpu.memory.write(0x1234, 0x55);
        cpu.registers.index_y = 0x34;

        cpu.run();

        assert_eq!(*cpu.registers.by_alias(target_alias), 0x55);
    }
}

pub mod ind {
    use super::Cpu;

    pub fn x_from_memory(opcode: u8, target_alias: char) {
        let mut cpu = Cpu::new();
        cpu.load_program(&[opcode, 0x0A]);

        cpu.memory.write(0x0010, 0x69);
        cpu.memory.write(0x0069, 0x55);
        cpu.registers.index_x = 0x06;

        cpu.run();

        assert_eq!(*cpu.registers.by_alias(target_alias), 0x55);
    }

    pub fn y_from_memory(opcode: u8, target_alias: char) {
        let mut cpu = Cpu::new();
        cpu.load_program(&[opcode, 0x10]);

        cpu.memory.write(0x0010, 0x60);
        cpu.memory.write(0x0069, 0x55);
        cpu.registers.index_y = 0x09;

        cpu.run();

        assert_eq!(*cpu.registers.by_alias(target_alias), 0x55);
    }
}
