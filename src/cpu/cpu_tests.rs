use super::{Status, CPU};

impl<'a> CPU<'a> {
    pub fn debug_load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.program_counter = self.mem_read_u16(0xFFFC);
        self.run();
    }
}

#[test]
fn lda_immidiate_load_data_accumulator() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
    assert_eq!(cpu.accumulator, 0x05);
    assert!(cpu.status.get() & Status::ZERO == 0b00);
    assert!(cpu.status.get() & Status::NEGATIV == 0);
}

#[test]
fn ldx_immidiate_load_data_register_x() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa2, 0x05, 0x00]);
    assert_eq!(cpu.register_x, 0x05);
    assert!(cpu.status.get() & Status::ZERO == 0b00);
    assert!(cpu.status.get() & Status::NEGATIV == 0);
}

#[test]
fn lda_zero_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
    assert!(cpu.status.get() & Status::ZERO == 0b10);
}

#[test]
fn tax_move_a_to_x() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0x0a, 0xaa, 0x00]);

    assert_eq!(cpu.register_x, 10);
}

#[test]
fn inx_increment_x() {
    let mut cpu = CPU::new();
    cpu.register_x = 0;
    cpu.load_and_run(vec![0xe8, 0x00]);

    assert_eq!(cpu.register_x, 1);
}

#[test]
fn inx_overflow() {
    let mut cpu = CPU::new();
    let mut program = vec![0xe8; 260];
    program.push(0x00);
    cpu.load_and_run(program);

    assert_eq!(cpu.register_x, 4)
}

#[test]
fn registers_set_to_0_after_reset() {
    let mut cpu = CPU::new();
    cpu.accumulator = 5;
    cpu.register_x = 6;
    cpu.register_y = 7;
    cpu.program_counter = 8;
    cpu.load_and_run(vec![0x00]);

    assert_eq!(cpu.accumulator, 0);
    assert_eq!(cpu.register_x, 0);
    assert_eq!(cpu.register_y, 0);
    assert_eq!(cpu.program_counter, 0x8001);
}

#[test]
fn test_5_ops_working_together() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

    assert_eq!(cpu.register_x, 0xc1)
}

#[test]
fn adc_basic() {
    let mut cpu = CPU::new();

    cpu.debug_load_and_run(vec![0xa9, 0x01, 0x69, 0x02, 0x00]);
    assert_eq!(cpu.accumulator, 3);

    cpu.reset();
    cpu.status.set(Status::CARRY);
    cpu.debug_load_and_run(vec![0xa9, 0x01, 0x69, 0x02, 0x00]);
    assert_eq!(cpu.accumulator, 4);
}

#[test]
fn adc_overflow_and_carry_flag() {
    let mut cpu = CPU::new();

    cpu.debug_load_and_run(vec![0xa9, 0x7F, 0x69, 0x01, 0x00]);
    assert_eq!(cpu.accumulator, 128);
    assert_eq!(cpu.status.get(), Status::NEGATIV | Status::OVERFLOW);

    cpu.reset();
    cpu.status.set(Status::CARRY);
    cpu.debug_load_and_run(vec![0xa9, 0xFF, 0x69, 0x01, 0x00]);
    assert_eq!(cpu.accumulator, 1);
    assert_eq!(cpu.status.get(), Status::CARRY)
}

#[test]
fn and_same_values() {
    let mut cpu = CPU::new();
    cpu.debug_load_and_run(vec![0xa9, 0x11, 0x29, 0x11, 0x00]);
    assert_eq!(cpu.accumulator, 0x11);
}

#[test]
fn and_different_values() {
    let mut cpu = CPU::new();
    cpu.debug_load_and_run(vec![0xa9, 0x11, 0x29, 0x01, 0x00]);
    assert_eq!(cpu.accumulator, 0x01);
}

#[test]
fn asl_number_in_accumulator() {
    let mut cpu = CPU::new();
    cpu.debug_load_and_run(vec![0xa9, 0x08, 0x0a, 0x00]);
    assert_eq!(cpu.accumulator, 0x10);
}

#[test]
fn asl_number_in_memory() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x00, 0x08);
    cpu.debug_load_and_run(vec![0x06, 0x00, 0x00]);
    assert_eq!(cpu.accumulator, 0x10);
}

#[test]
fn asl_carry_and_negative_flag() {
    let mut cpu = CPU::new();
    cpu.debug_load_and_run(vec![0xa9, 0xFF, 0x0a, 0x00]);
    assert_eq!(cpu.accumulator, 0xFE);
    assert_eq!(cpu.status.get(), Status::NEGATIV | Status::CARRY);
}

#[test]
fn clc_clear_carry_flag() {
    let mut cpu = CPU::new();
    cpu.status.set(Status::CARRY);
    cpu.debug_load_and_run(vec![0x18, 0x00]);
    assert_eq!(cpu.status.get(), 0x00);
}

#[test]
fn sec_set_carry_flag() {
    let mut cpu = CPU::new();
    cpu.debug_load_and_run(vec![0x38, 0x00]);
    assert_eq!(cpu.status.get(), Status::CARRY);
}

#[test]
fn cld_clear_decimal_flag() {
    let mut cpu = CPU::new();
    cpu.status.set(Status::DECIMAL_MODE);
    cpu.debug_load_and_run(vec![0xD8, 0x00]);
    assert_eq!(cpu.status.get(), 0x00);
}

#[test]
fn sed_set_decimal_flag() {
    let mut cpu = CPU::new();
    cpu.debug_load_and_run(vec![0xF8, 0x00]);
    assert_eq!(cpu.status.get(), Status::DECIMAL_MODE);
}

#[test]
fn cli_clear_interrupt_disable_flag() {
    let mut cpu = CPU::new();
    cpu.status.set(Status::INTERRUPT_DISABLE);
    cpu.debug_load_and_run(vec![0x58, 0x00]);
    assert_eq!(cpu.status.get(), 0x00);
}

#[test]
fn sei_set_interrupt_disable_flag() {
    let mut cpu = CPU::new();
    cpu.debug_load_and_run(vec![0x78, 0x00]);
    assert_eq!(cpu.status.get(), Status::INTERRUPT_DISABLE);
}

#[test]
fn clv_clear_overflow_flag() {
    let mut cpu = CPU::new();
    cpu.status.set(Status::OVERFLOW);
    cpu.debug_load_and_run(vec![0xB8, 0x00]);
    assert_eq!(cpu.status.get(), 0x00);
}

#[test]
fn cmp_with_smaller_number() {
    let mut cpu = CPU::new();
    cpu.debug_load_and_run(vec![0xa9, 0x05, 0xc9, 0x04, 0x00]);
    assert_eq!(cpu.status.get(), Status::CARRY);
}

#[test]
fn cmp_with_bigger_number() {
    let mut cpu = CPU::new();
    cpu.debug_load_and_run(vec![0xa9, 0x05, 0xc9, 0x06, 0x00]);
    assert_eq!(cpu.status.get(), Status::NEGATIV);
}

#[test]
fn cmp_with_same_number() {
    let mut cpu = CPU::new();
    cpu.debug_load_and_run(vec![0xa9, 0x05, 0xc9, 0x05, 0x00]);
    assert_eq!(cpu.status.get(), Status::ZERO | Status::CARRY);
}

#[test]
fn cpx_with_bigger_number() {
    let mut cpu = CPU::new();
    cpu.debug_load_and_run(vec![0xa2, 0x05, 0xe0, 0x06, 0x00]);
    assert_eq!(cpu.status.get(), Status::NEGATIV);
}

#[test]
fn cpy_with_bigger_number() {
    let mut cpu = CPU::new();
    cpu.debug_load_and_run(vec![0xa2, 0x05, 0xc0, 0x06, 0x00]);
    assert_eq!(cpu.status.get(), Status::NEGATIV);
}

#[test]
fn dex_decrement_register_x() {
    let mut cpu = CPU::new();
    cpu.register_x = 1;
    cpu.debug_load_and_run(vec![0xca, 0x00]);
    assert_eq!(cpu.register_x, 0);
    assert_eq!(cpu.status.get(), Status::ZERO);
}

#[test]
fn dey_decrement_register_y() {
    let mut cpu = CPU::new();
    cpu.register_y = 1;
    cpu.debug_load_and_run(vec![0x88, 0x00]);
    assert_eq!(cpu.register_y, 0);
    assert_eq!(cpu.status.get(), Status::ZERO);
}
