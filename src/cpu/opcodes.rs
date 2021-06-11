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
            mnemonic: "NUL",
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
            0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => self.ldx(opcode),

            0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => self.adc(opcode),

            0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => self.and(opcode),

            0xC9 | 0xC5 | 0xD5 | 0xCD | 0xDD | 0xD9 | 0xC1 | 0xD1 => self.cmp(opcode),

            0xE0 | 0xE4 | 0xEC => self.cpx(opcode),

            0xC0 | 0xC4 | 0xCC => self.cpy(opcode),

            0x0A | 0x06 | 0x16 | 0x0E | 0x1E => self.asl(opcode),

            0x18 => self.clc(),
            0x38 => self.sec(),
            
            0xD8 => self.cld(),
            0xF8 => self.sed(),

            0x58 => self.cli(),
            0x78 => self.sei(),

            0xB8 => self.clv(),

            0xCA => self.dex(),
            0x88 => self.dey(),

            0xAA => self.tax(opcode),

            0xE8 => self.inx(opcode),

            _ => panic!("Unknown opcode: {:#x}", opcode.code),
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

    fn asl(&mut self, opcode: &Opcode) {
        let value;
        if opcode.mode == AddressingMode::None {
            value = self.accumulator;
        } else {
            let address = self.get_operand_address(opcode.mode);
            value = self.mem_read(address);  
        }

        let result = (value as u16) << 1;
        if result > 0xff {
            self.status.set(Status::CARRY)
        }

        self.accumulator = result as u8;
        self.update_zero_and_negative_flags(self.accumulator);
        self.increment_program_counter(opcode.length);
    }

    fn cmp(&mut self, opcode: &Opcode) {
        self.compare(opcode, self.accumulator);
    }

    fn cpx(&mut self, opcode: &Opcode) {
        self.compare(opcode, self.register_x);
    }

    fn cpy(&mut self, opcode: &Opcode) {
        self.compare(opcode, self.register_y);
    }

    fn clc(&mut self) {
        self.status.reset(Status::CARRY);
    }

    fn sec(&mut self) {
        self.status.set(Status::CARRY);
    }

    fn cld(&mut self) {
        self.status.reset(Status::DECIMAL_MODE);
    }

    fn sed(&mut self) {
        self.status.set(Status::DECIMAL_MODE);
    }

    fn cli(&mut self) {
        self.status.reset(Status::INTERRUPT_DISABLE);
    }

    fn sei(&mut self) {
        self.status.set(Status::INTERRUPT_DISABLE);
    }

    fn clv(&mut self) {
        self.status.reset(Status::OVERFLOW);
    }

    fn dex(&mut self) {
        self.register_x = self.register_x.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn dey(&mut self) {
        self.register_y = self.register_y.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.register_y);
    }

    fn lda(&mut self, opcode: &Opcode) {
        let address = self.get_operand_address(opcode.mode);
        let value = self.mem_read(address);

        self.accumulator = value;
        self.update_zero_and_negative_flags(self.accumulator);
        self.increment_program_counter(opcode.length);
    }

    fn ldx(&mut self, opcode: &Opcode) {
        let address = self.get_operand_address(opcode.mode);
        let value = self.mem_read(address);

        self.register_x = value;
        self.update_zero_and_negative_flags(self.register_x);
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

    fn compare(&mut self, opcode: &Opcode, register: u8) {
        let address = self.get_operand_address(opcode.mode);
        let value = self.mem_read(address);

        if register >= value {
            self.status.set(Status::CARRY);
        } else {
            self.status.reset(Status::CARRY);
        }

        self.update_zero_and_negative_flags(register.wrapping_sub(value));
        self.increment_program_counter(opcode.length);
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

        opcode_table[0xA2] = Opcode::new(0xA2, "LDX", 2, 2, AddressingMode::Immediate);
        opcode_table[0xA6] = Opcode::new(0xA6, "LDX", 2, 3, AddressingMode::ZeroPage);
        opcode_table[0xB6] = Opcode::new(0xB6, "LDX", 2, 4, AddressingMode::ZeroPage_X);
        opcode_table[0xAE] = Opcode::new(0xAE, "LDX", 3, 4, AddressingMode::Absolute);
        opcode_table[0xBE] = Opcode::new(0xBE, "LDX", 3, 4, AddressingMode::Absolute_Y);

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

        opcode_table[0xC9] = Opcode::new(0xC9, "CMP", 2, 2, AddressingMode::Immediate);
        opcode_table[0xC5] = Opcode::new(0xC5, "CMP", 2, 3, AddressingMode::ZeroPage);
        opcode_table[0xD5] = Opcode::new(0xD5, "CMP", 2, 4, AddressingMode::ZeroPage_X);
        opcode_table[0xCD] = Opcode::new(0xCD, "CMP", 3, 4, AddressingMode::Absolute);
        opcode_table[0xDD] = Opcode::new(0xDD, "CMP", 3, 4, AddressingMode::Absolute_X);
        opcode_table[0xD9] = Opcode::new(0xD9, "CMP", 3, 4, AddressingMode::Absolute_Y);
        opcode_table[0xC1] = Opcode::new(0xC1, "CMP", 2, 6, AddressingMode::Indirect_X);
        opcode_table[0xD1] = Opcode::new(0xD1, "CMP", 2, 5, AddressingMode::Indirect_Y);

        opcode_table[0xE0] = Opcode::new(0xE0, "CPX", 2, 2, AddressingMode::Immediate);
        opcode_table[0xE4] = Opcode::new(0xE4, "CPX", 2, 3, AddressingMode::ZeroPage);
        opcode_table[0xEC] = Opcode::new(0xEC, "CPX", 3, 4, AddressingMode::Absolute);

        opcode_table[0xC0] = Opcode::new(0xC0, "CPY", 2, 2, AddressingMode::Immediate);
        opcode_table[0xC4] = Opcode::new(0xC4, "CPY", 2, 3, AddressingMode::ZeroPage);
        opcode_table[0xCC] = Opcode::new(0xCC, "CPY", 3, 4, AddressingMode::Absolute);

        opcode_table[0x0A] = Opcode::new(0x0A, "ASL", 1, 2, AddressingMode::None);
        opcode_table[0x06] = Opcode::new(0x06, "ASL", 2, 5, AddressingMode::ZeroPage);
        opcode_table[0x16] = Opcode::new(0x16, "ASL", 2, 6, AddressingMode::ZeroPage_X);
        opcode_table[0x0E] = Opcode::new(0x0E, "ASL", 1, 6, AddressingMode::Absolute);
        opcode_table[0x1E] = Opcode::new(0x1E, "ASL", 1, 7, AddressingMode::Absolute_X);

        opcode_table[0x18] = Opcode::new(0x18, "CLC", 1, 2, AddressingMode::None);
        opcode_table[0x38] = Opcode::new(0x38, "SEC", 1, 2, AddressingMode::None);

        opcode_table[0xD8] = Opcode::new(0xD8, "CLD", 1, 2, AddressingMode::None);
        opcode_table[0xF8] = Opcode::new(0xF8, "SED", 1, 2, AddressingMode::None);

        opcode_table[0x58] = Opcode::new(0x58, "CLI", 1, 2, AddressingMode::None);
        opcode_table[0x78] = Opcode::new(0x78, "SEI", 1, 2, AddressingMode::None);

        opcode_table[0xB8] = Opcode::new(0xB8, "CLV", 1, 2, AddressingMode::None);

        opcode_table[0xCA] = Opcode::new(0xCA, "DEX", 1, 2, AddressingMode::None);
        opcode_table[0x88] = Opcode::new(0x88, "DEY", 1, 2, AddressingMode::None);

        opcode_table[0xAA] = Opcode::new(0xAA, "TAX", 1, 2, AddressingMode::None);

        opcode_table[0xE8] = Opcode::new(0xE8, "INX", 1, 2, AddressingMode::None);

        opcode_table[0x00] = Opcode::new(0x00, "BRK", 1, 7, AddressingMode::None);

        opcode_table
    }
}
