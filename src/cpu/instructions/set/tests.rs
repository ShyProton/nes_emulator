use super::aliases::RegisterAlias;

#[test]
fn conglomerate_1() {
    use super::Cpu;

    let mut cpu = Cpu::new();
    cpu.load_program(&[0xA9, 0xC0, 0xAA, 0xE8, 0x00]);
    cpu.run();

    assert_eq!(cpu.registers.get_register(&RegisterAlias::X), 0xC1);
}
