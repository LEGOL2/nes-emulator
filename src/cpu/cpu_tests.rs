use super::CPU;

#[test]
fn lda_immidiate_load_data_accumulator() {
    let mut cpu = CPU::new();
    cpu.interpret(vec![0xa9, 0x05, 0x00]);
    assert_eq!(cpu.accumulator, 0x05);
    assert!(cpu.status & 0b0000_0010 == 0b00);
    assert!(cpu.status & 0b1000_0000 == 0);
}

#[test]
fn lda_zero_flag() {
    let mut cpu = CPU::new();
    cpu.interpret(vec![0xa9, 0x00, 0x00]);
    assert!(cpu.status & 0b0000_0010 == 0b10);
}

#[test]
fn tax_move_a_to_x() {
    let mut cpu = CPU::new();
    cpu.accumulator = 10;
    cpu.interpret(vec![0xaa, 0x00]);

    assert_eq!(cpu.register_x, 10);
}

#[test]
fn inx_increment_x() {
    let mut cpu = CPU::new();
    cpu.register_x = 0;
    cpu.interpret(vec![0xe8, 0x00]);

    assert_eq!(cpu.register_x, 1);
}

#[test]
fn inx_overflow() {
    let mut cpu = CPU::new();
    cpu.register_x = 0xff;
    cpu.interpret(vec![0xe8, 0xe8, 0x00]);

    assert_eq!(cpu.register_x, 1)
}

#[test]
fn test_5_ops_working_together() {
    let mut cpu = CPU::new();
    cpu.interpret(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

    assert_eq!(cpu.register_x, 0xc1)
}


