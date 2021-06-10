use super::{AddressingMode, Status, CPU};

#[derive(Clone, Copy)]
pub struct Opcode<'a> {
    pub code: u8,
    pub mnemonic: &'a str,
    pub length: u8,
    pub cycles: u8,
    pub mode: AddressingMode,
}

impl<'a> Opcode<'a> {
    fn new(code: u8, mnemonic: &'a str, length: u8, cycles: u8, mode: AddressingMode) -> Self {
        Opcode {
            code,
            mnemonic,
            length,
            cycles,
            mode,
        }
    }

    fn basic() -> Self {
        Opcode {
            code: 0,
            mnemonic: "",
            length: 0,
            cycles: 0,
            mode: AddressingMode::Immediate,
        }
    }
}

impl<'a> CPU<'a> {
    pub fn interpret(&mut self, opcode: &Opcode) -> bool {
        match opcode.code {
            0x00 => {
                self.increment_program_counter(opcode.length);
                return false;
            }

            0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => self.lda(opcode),

            0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => self.adc(opcode),

            0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => self.and(opcode),

            0xAA => self.tax(opcode),

            0xE8 => self.inx(opcode),

            _ => panic!("Unknown opcode!"),
        }

        true
    }

    fn adc(&mut self, opcode: &Opcode) {
        let address = self.get_operand_address(opcode.mode);
        let value = self.mem_read(address);
        self.add_to_accumulator(value);
        self.update_zero_and_negative_flags(self.accumulator);
        self.increment_program_counter(opcode.length);
    }

    fn and(&mut self, opcode: &Opcode) {
        let address = self.get_operand_address(opcode.mode);
        let value = self.mem_read(address);
        self.accumulator &= value;
        self.update_zero_and_negative_flags(self.accumulator);
        self.increment_program_counter(opcode.length);
    }

    fn lda(&mut self, opcode: &Opcode) {
        let address = self.get_operand_address(opcode.mode);
        let value = self.mem_read(address);

        self.accumulator = value;
        self.update_zero_and_negative_flags(self.accumulator);
        self.increment_program_counter(opcode.length);
    }

    fn tax(&mut self, opcode: &Opcode) {
        self.register_x = self.accumulator;
        self.update_zero_and_negative_flags(self.register_x);
        self.increment_program_counter(opcode.length);
    }

    fn inx(&mut self, opcode: &Opcode) {
        if self.register_x == std::u8::MAX {
            self.register_x = 0;
        } else {
            self.register_x += 1;
        }
        self.update_zero_and_negative_flags(self.register_x);
        self.increment_program_counter(opcode.length);
    }

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            self.status.set(Status::ZERO);
        } else {
            self.status.reset(Status::ZERO);
        }

        if result & 0b1000_0000 != 0 {
            self.status.set(Status::NEGATIV);
        } else {
            self.status.reset(Status::NEGATIV);
        }
    }

    fn add_to_accumulator(&mut self, data: u8) {
        let carry = if self.status.get() & 0x01 == 1 { 1 } else { 0 };
        let sum = self.accumulator as u16 + data as u16 + carry;

        if sum > 0xff {
            self.status.set(Status::CARRY);
        } else {
            self.status.reset(Status::CARRY);
        }

        let result = sum as u8;

        if (data ^ result) & (result ^ self.accumulator) & 0x80 != 0 {
            self.status.set(Status::OVERFLOW);
        } else {
            self.status.reset(Status::OVERFLOW);
        }

        self.accumulator = result;
    }

    pub fn create_opcode_table() -> [Opcode<'a>; 0xFF] {
        let mut opcode_table: [Opcode; 0xFF] = [Opcode::basic(); 0xFF];

        opcode_table[0xA9] = Opcode::new(0xA9, "LDA", 2, 2, AddressingMode::Immediate);
        opcode_table[0xA5] = Opcode::new(0xA5, "LDA", 2, 3, AddressingMode::ZeroPage);
        opcode_table[0xB5] = Opcode::new(0xB5, "LDA", 2, 4, AddressingMode::ZeroPage_X);
        opcode_table[0xAD] = Opcode::new(0xAD, "LDA", 3, 4, AddressingMode::Absolute);
        opcode_table[0xBD] = Opcode::new(0xBD, "LDA", 3, 4, AddressingMode::Absolute_X);
        opcode_table[0xB9] = Opcode::new(0xB9, "LDA", 3, 4, AddressingMode::Absolute_Y);
        opcode_table[0xA1] = Opcode::new(0xA1, "LDA", 2, 6, AddressingMode::Indirect_X);
        opcode_table[0xB1] = Opcode::new(0xB1, "LDA", 2, 5, AddressingMode::Indirect_Y);

        opcode_table[0x69] = Opcode::new(0x69, "ADC", 2, 2, AddressingMode::Immediate);
        opcode_table[0x65] = Opcode::new(0x65, "ADC", 2, 3, AddressingMode::ZeroPage);
        opcode_table[0x75] = Opcode::new(0x75, "ADC", 2, 4, AddressingMode::ZeroPage_X);
        opcode_table[0x6D] = Opcode::new(0x6D, "ADC", 3, 4, AddressingMode::Absolute);
        opcode_table[0x7D] = Opcode::new(0x7D, "ADC", 3, 4, AddressingMode::Absolute_X);
        opcode_table[0x79] = Opcode::new(0x79, "ADC", 3, 4, AddressingMode::Absolute_Y);
        opcode_table[0x61] = Opcode::new(0x61, "ADC", 2, 6, AddressingMode::Indirect_X);
        opcode_table[0x71] = Opcode::new(0x71, "ADC", 2, 5, AddressingMode::Indirect_Y);

        opcode_table[0x29] = Opcode::new(0x29, "AND", 2, 2, AddressingMode::Immediate);
        opcode_table[0x25] = Opcode::new(0x25, "AND", 2, 3, AddressingMode::ZeroPage);
        opcode_table[0x35] = Opcode::new(0x35, "AND", 2, 4, AddressingMode::ZeroPage_X);
        opcode_table[0x2D] = Opcode::new(0x2D, "AND", 3, 4, AddressingMode::Absolute);
        opcode_table[0x3D] = Opcode::new(0x3D, "AND", 3, 4, AddressingMode::Absolute_X);
        opcode_table[0x39] = Opcode::new(0x39, "AND", 3, 4, AddressingMode::Absolute_Y);
        opcode_table[0x21] = Opcode::new(0x21, "AND", 2, 6, AddressingMode::Indirect_X);
        opcode_table[0x31] = Opcode::new(0x31, "AND", 2, 5, AddressingMode::Indirect_Y);

        opcode_table[0xAA] = Opcode::new(0xAA, "TAX", 1, 2, AddressingMode::None);

        opcode_table[0xE8] = Opcode::new(0xE8, "INX", 1, 2, AddressingMode::None);

        opcode_table[0x00] = Opcode::new(0x00, "BRK", 1, 7, AddressingMode::None);

        opcode_table
    }
}
