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
