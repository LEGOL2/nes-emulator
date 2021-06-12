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
            code: 0x02,
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

            0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => self.adc(opcode),

            0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => self.and(opcode),

            0xC9 | 0xC5 | 0xD5 | 0xCD | 0xDD | 0xD9 | 0xC1 | 0xD1 => self.cmp(opcode),

            0xE0 | 0xE4 | 0xEC => self.cpx(opcode),

            0xC0 | 0xC4 | 0xCC => self.cpy(opcode),

            0x0A | 0x06 | 0x16 | 0x0E | 0x1E => self.asl(opcode),

            0x18 => self.clc(),
            0xD8 => self.cld(),
            0x58 => self.cli(),
            0xB8 => self.clv(),

            0xC6 | 0xD6 | 0xCE | 0xDE => self.dec(opcode),
            0xCA => self.dex(),
            0x88 => self.dey(),

            0x49 | 0x45 | 0x55 | 0x4D | 0x5D | 0x59 | 0x41 | 0x51 => self.eor(opcode),
            
            0xE6 | 0xF6 | 0xEE | 0xFE => self.inc(opcode),
            0xE8 => self.inx(opcode),
            0xC8 => self.iny(opcode),

            0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => self.lda(opcode),
            0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => self.ldx(opcode),
            0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => self.ldy(opcode),
            
            0x4A | 0x46 | 0x56 | 0x4E | 0x5E => self.lsr(opcode),

            0xEA => self.nop(),

            0x09 | 0x05 | 0x15 | 0x0D | 0x1D | 0x19 | 0x01 | 0x11 => self.ora(opcode),

            0x48 => self.pha(),
            0x08 => self.php(),
            0x68 => self.pla(),
            0x28 => self.plp(),
 |          
            0x2A | 0x26 | 0x36 | 0x2E | 0x3E => self.rol(opcode),
            0x6A | 0x66 | 0x76 | 0x6E | 0x7E => self.ror(opcode),

            0xE9 | 0xE5 | 0xF5 | 0xED | 0xFD | 0xF9 | 0xE1 | 0xF1 => self.sbc(opcode),

            0x38 => self.sec(),
            0xF8 => self.sed(),
            0x78 => self.sei(),

            0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => self.sta(opcode),           
            0x86 | 0x96 | 0x8E => self.stx(opcode),
            0x84 | 0x94 | 0x8C => self.sty(opcode),

            0xAA => self.tax(),
            0xA8 => self.tay(),
            0xBA => self.tsx(),
            0x8A => self.txa(),
            0x9A => self.txs(),
            0x98 => self.tya(),

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

    fn dec(&mut self, opcode: &Opcode) {
        let address = self.get_operand_address(opcode.mode);
        let value = self.mem_read(address);
        let result = value.wrapping_sub(1);
        self.mem_write(address, result);
        self.update_zero_and_negative_flags(result);
        self.increment_program_counter(opcode.length);
    }

    fn dex(&mut self) {
        self.register_x = self.register_x.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn dey(&mut self) {
        self.register_y = self.register_y.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.register_y);
    }

    fn eor(&mut self, opcode: &Opcode) {
        let address = self.get_operand_address(opcode.mode);
        let value = self.mem_read(address);
        let result = self.accumulator ^ value;
        self.accumulator = result;
        self.update_zero_and_negative_flags(result);
        self.increment_program_counter(opcode.length);
    }

    fn inc(&mut self, opcode: &Opcode) {
        let address = self.get_operand_address(opcode.mode);
        let value = self.mem_read(address);
        let result = value.wrapping_add(1);
        self.mem_write(address, result);
        self.update_zero_and_negative_flags(result);
        self.increment_program_counter(opcode.length);
    }

    fn inx(&mut self, opcode: &Opcode) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_x);
        self.increment_program_counter(opcode.length);
    }

    fn iny(&mut self, opcode: &Opcode) {
        self.register_y = self.register_y.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_y);
        self.increment_program_counter(opcode.length);
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

    fn ldy(&mut self, opcode: &Opcode) {
        let address = self.get_operand_address(opcode.mode);
        let value = self.mem_read(address);

        self.register_y = value;
        self.update_zero_and_negative_flags(self.register_y);
        self.increment_program_counter(opcode.length);
    }

    fn lsr(&mut self, opcode: &Opcode) {
        let before;
        if opcode.mode == AddressingMode::None {
            before = self.accumulator;
            self.accumulator = self.accumulator >> 1;
        } else {
            let address = self.get_operand_address(opcode.mode);
            let value = self.mem_read(address);
            before = value;
            self.mem_write(address, value >> 1);
        }

        let carry = before & 0x01;
        if carry == 1 {
            self.status.set(Status::CARRY);
        } else {
            self.status.reset(Status::CARRY);
        }

        self.update_zero_and_negative_flags(before >> 1);
        self.increment_program_counter(opcode.length);
    }

    fn nop(&self) {}

    fn ora (&mut self, opcode: &Opcode) {
        let address = self.get_operand_address(opcode.mode);
        let value = self.mem_read(address);

        self.accumulator |= value;
        self.update_zero_and_negative_flags(self.accumulator);
        self.increment_program_counter(opcode.length);
    }

    fn pha(&mut self) {
        self.push(self.accumulator);
    }

    fn php(&mut self) {
        self.push(self.status.get());
    }

    fn pla(&mut self) {
        self.accumulator = self.pop();
        self.update_zero_and_negative_flags(self.accumulator);
    }

    fn plp(&mut self) {
        let data = self.pop();
        self.status.insert(data);
    }

    fn rol(&mut self, opcode: &Opcode) {
        let carry;
        let result;
        if opcode.mode == AddressingMode::None {
            carry = self.accumulator & 0x80;
            self.accumulator = self.accumulator << 1;
            self.accumulator |= self.status.get() & Status::CARRY;
            result = self.accumulator;
        } else {
            let address = self.get_operand_address(opcode.mode);
            let mut value = self.mem_read(address);
            carry = value & 0x80;
            value = value << 1;
            value |= self.status.get() & Status::CARRY;
            self.mem_write(address, value);
            result = value;
        }

        if carry == 0x80 {
            self.status.set(Status::CARRY);
        } else {
            self.status.reset(Status::CARRY);
        }
        if result & 0x80 == 0x80 {
            self.status.set(Status::NEGATIV);
        } else {
            self.status.reset(Status::NEGATIV);
        }
        if self.accumulator == 0 {
            self.status.set(Status::ZERO);
        } else {
            self.status.reset(Status::ZERO);
        }
        self.increment_program_counter(opcode.length);
    }

    fn ror(&mut self, opcode: &Opcode) {
        let carry;
        let result;
        if opcode.mode == AddressingMode::None {
            carry = self.accumulator & 0x01;
            self.accumulator = self.accumulator >> 1;
            let old_carry = self.status.get() & Status::CARRY;
            if old_carry == 1 {
                self.accumulator |= 0x80;
            }
            result = self.accumulator;
        } else {
            let address = self.get_operand_address(opcode.mode);
            let mut value = self.mem_read(address);
            carry = value & 0x01;
            value = value >> 1;
            let old_carry = self.status.get() & Status::CARRY;
            if old_carry == 1 {
                value |= 0x80;
            }
            self.mem_write(address, value);
            result = value;
        }

        if carry == 1 {
            self.status.set(Status::CARRY);
        } else {
            self.status.reset(Status::CARRY);
        }
        if result & 0x80 == 0x80 {
            self.status.set(Status::NEGATIV);
        } else {
            self.status.reset(Status::NEGATIV);
        }
        if self.accumulator == 0 {
            self.status.set(Status::ZERO);
        } else {
            self.status.reset(Status::ZERO);
        }
        self.increment_program_counter(opcode.length);
    }

    fn sbc(&mut self, opcode: &Opcode) {
        let address = self.get_operand_address(opcode.mode);
        let mut value = self.mem_read(address);
        value = !value + 1;
        self.add_to_accumulator(value);
        self.update_zero_and_negative_flags(self.accumulator);
        self.increment_program_counter(opcode.length);
    }

    fn sta(&mut self, opcode: &Opcode) {
        let address = self.get_operand_address(opcode.mode);
        self.mem_write(address, self.accumulator);
        self.increment_program_counter(opcode.length);
    }
    
    fn stx(&mut self, opcode: &Opcode) {
        let address = self.get_operand_address(opcode.mode);
        self.mem_write(address, self.register_x);
        self.increment_program_counter(opcode.length);
    }

    fn sty(&mut self, opcode: &Opcode) {
        let address = self.get_operand_address(opcode.mode);
        self.mem_write(address, self.register_y);
        self.increment_program_counter(opcode.length);
    }

    fn tax(&mut self) {
        self.register_x = self.accumulator;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn tay(&mut self) {
        self.register_y = self.accumulator;
        self.update_zero_and_negative_flags(self.register_y);
    }

    fn tsx(&mut self) {
        self.register_x = self.stack_pointer as u8;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn txa(&mut self) {
        self.accumulator = self.register_x;
        self.update_zero_and_negative_flags(self.accumulator);
    }

    fn txs(&mut self) {
        self.stack_pointer = (self.register_x as u16) | 0x100;
    }

    fn tya(&mut self) {
        self.accumulator = self.register_y;
        self.update_zero_and_negative_flags(self.accumulator);
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

        opcode_table[0x0A] = Opcode::new(0x0A, "ASL", 1, 2, AddressingMode::None);
        opcode_table[0x06] = Opcode::new(0x06, "ASL", 2, 5, AddressingMode::ZeroPage);
        opcode_table[0x16] = Opcode::new(0x16, "ASL", 2, 6, AddressingMode::ZeroPage_X);
        opcode_table[0x0E] = Opcode::new(0x0E, "ASL", 1, 6, AddressingMode::Absolute);
        opcode_table[0x1E] = Opcode::new(0x1E, "ASL", 1, 7, AddressingMode::Absolute_X);

        opcode_table[0x00] = Opcode::new(0x00, "BRK", 1, 7, AddressingMode::None);

        opcode_table[0x18] = Opcode::new(0x18, "CLC", 1, 2, AddressingMode::None);
        opcode_table[0xD8] = Opcode::new(0xD8, "CLD", 1, 2, AddressingMode::None);
        opcode_table[0x58] = Opcode::new(0x58, "CLI", 1, 2, AddressingMode::None);
        opcode_table[0xB8] = Opcode::new(0xB8, "CLV", 1, 2, AddressingMode::None);

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

        opcode_table[0xC6] = Opcode::new(0xC6, "DEC", 2, 5, AddressingMode::ZeroPage);
        opcode_table[0xD6] = Opcode::new(0xD6, "DEC", 2, 6, AddressingMode::ZeroPage_X);
        opcode_table[0xCE] = Opcode::new(0xCE, "DEC", 3, 6, AddressingMode::Absolute);
        opcode_table[0xDE] = Opcode::new(0xDE, "DEC", 3, 7, AddressingMode::Absolute_X);

        opcode_table[0xCA] = Opcode::new(0xCA, "DEX", 1, 2, AddressingMode::None);
        opcode_table[0x88] = Opcode::new(0x88, "DEY", 1, 2, AddressingMode::None);

        opcode_table[0x49] = Opcode::new(0x49, "EOR", 2, 2, AddressingMode::Immediate);
        opcode_table[0x45] = Opcode::new(0x45, "EOR", 2, 3, AddressingMode::ZeroPage);
        opcode_table[0x55] = Opcode::new(0x55, "EOR", 2, 4, AddressingMode::ZeroPage_X);
        opcode_table[0x4D] = Opcode::new(0x4D, "EOR", 3, 4, AddressingMode::Absolute);
        opcode_table[0x5D] = Opcode::new(0x5D, "EOR", 3, 4, AddressingMode::Absolute_X);
        opcode_table[0x59] = Opcode::new(0x59, "EOR", 3, 4, AddressingMode::Absolute_Y);
        opcode_table[0x41] = Opcode::new(0x41, "EOR", 2, 6, AddressingMode::Indirect_X);
        opcode_table[0x51] = Opcode::new(0x51, "EOR", 2, 5, AddressingMode::Indirect_Y);

        opcode_table[0xE6] = Opcode::new(0xE6, "INC", 2, 5, AddressingMode::ZeroPage);
        opcode_table[0xF6] = Opcode::new(0xF6, "INC", 2, 6, AddressingMode::ZeroPage_X);
        opcode_table[0xEE] = Opcode::new(0xEE, "INC", 3, 6, AddressingMode::Absolute);
        opcode_table[0xFE] = Opcode::new(0xFE, "INC", 3, 7, AddressingMode::Absolute_X);
        
        opcode_table[0xE8] = Opcode::new(0xE8, "INX", 1, 2, AddressingMode::None);
        opcode_table[0xC8] = Opcode::new(0xC8, "INY", 1, 2, AddressingMode::None);

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

        opcode_table[0xA0] = Opcode::new(0xA0, "LDY", 2, 2, AddressingMode::Immediate);
        opcode_table[0xA4] = Opcode::new(0xA4, "LDY", 2, 3, AddressingMode::ZeroPage);
        opcode_table[0xB4] = Opcode::new(0xB4, "LDY", 2, 4, AddressingMode::ZeroPage_X);
        opcode_table[0xAC] = Opcode::new(0xAC, "LDY", 3, 4, AddressingMode::Absolute);
        opcode_table[0xBC] = Opcode::new(0xBC, "LDY", 3, 4, AddressingMode::Absolute_Y);

        opcode_table[0x4A] = Opcode::new(0x4A, "LSR", 1, 2, AddressingMode::None);
        opcode_table[0x46] = Opcode::new(0x46, "LSR", 2, 5, AddressingMode::ZeroPage);
        opcode_table[0x56] = Opcode::new(0x56, "LSR", 2, 6, AddressingMode::ZeroPage_X);
        opcode_table[0x4E] = Opcode::new(0x4E, "LSR", 3, 6, AddressingMode::Absolute);
        opcode_table[0x5E] = Opcode::new(0x5E, "LSR", 3, 7, AddressingMode::Absolute_X);

        opcode_table[0xEA] = Opcode::new(0xEA, "NOP", 1, 2, AddressingMode::None);

        opcode_table[0x09] = Opcode::new(0x09, "ORA", 2, 2, AddressingMode::Immediate);
        opcode_table[0x05] = Opcode::new(0x05, "ORA", 2, 3, AddressingMode::ZeroPage);
        opcode_table[0x15] = Opcode::new(0x15, "ORA", 2, 4, AddressingMode::ZeroPage_X);
        opcode_table[0x0D] = Opcode::new(0x0D, "ORA", 3, 4, AddressingMode::Absolute);
        opcode_table[0x1D] = Opcode::new(0x1D, "ORA", 3, 4, AddressingMode::Absolute_X);
        opcode_table[0x19] = Opcode::new(0x19, "ORA", 3, 4, AddressingMode::Absolute_Y);
        opcode_table[0x01] = Opcode::new(0x01, "ORA", 2, 6, AddressingMode::Indirect_X);
        opcode_table[0x11] = Opcode::new(0x11, "ORA", 2, 5, AddressingMode::Indirect_Y);

        opcode_table[0x48] = Opcode::new(0x48, "PHA", 1, 3, AddressingMode::None);
        opcode_table[0x08] = Opcode::new(0x08, "PHP", 1, 3, AddressingMode::None);
        opcode_table[0x68] = Opcode::new(0x68, "PLA", 1, 4, AddressingMode::None);
        opcode_table[0x28] = Opcode::new(0x28, "PLP", 1, 4, AddressingMode::None);

        opcode_table[0x2A] = Opcode::new(0x2A, "ROL", 1, 2, AddressingMode::None);
        opcode_table[0x26] = Opcode::new(0x26, "ROL", 2, 5, AddressingMode::ZeroPage);
        opcode_table[0x36] = Opcode::new(0x36, "ROL", 2, 6, AddressingMode::ZeroPage_X);
        opcode_table[0x2E] = Opcode::new(0x2E, "ROL", 3, 6, AddressingMode::Absolute);
        opcode_table[0x3E] = Opcode::new(0x3E, "ROL", 3, 7, AddressingMode::Absolute_X);

        opcode_table[0x6A] = Opcode::new(0x6A, "ROR", 1, 2, AddressingMode::None);
        opcode_table[0x66] = Opcode::new(0x66, "ROR", 2, 5, AddressingMode::ZeroPage);
        opcode_table[0x76] = Opcode::new(0x76, "ROR", 2, 6, AddressingMode::ZeroPage_X);
        opcode_table[0x6E] = Opcode::new(0x6E, "ROR", 3, 6, AddressingMode::Absolute);
        opcode_table[0x7E] = Opcode::new(0x7E, "ROR", 3, 7, AddressingMode::Absolute_X);

        opcode_table[0xE9] = Opcode::new(0xE9, "SBC", 2, 2, AddressingMode::Immediate);
        opcode_table[0xE5] = Opcode::new(0xE5, "SBC", 2, 3, AddressingMode::ZeroPage);
        opcode_table[0xF5] = Opcode::new(0xF5, "SBC", 2, 4, AddressingMode::ZeroPage_X);
        opcode_table[0xED] = Opcode::new(0xED, "SBC", 3, 4, AddressingMode::Absolute);
        opcode_table[0xFD] = Opcode::new(0xFD, "SBC", 3, 4, AddressingMode::Absolute_X);
        opcode_table[0xF9] = Opcode::new(0xF9, "SBC", 3, 4, AddressingMode::Absolute_Y);
        opcode_table[0xE1] = Opcode::new(0xE1, "SBC", 2, 6, AddressingMode::Indirect_X);
        opcode_table[0xF1] = Opcode::new(0xF1, "SBC", 2, 5, AddressingMode::Indirect_Y);

        opcode_table[0x38] = Opcode::new(0x38, "SEC", 1, 2, AddressingMode::None);
        opcode_table[0xF8] = Opcode::new(0xF8, "SED", 1, 2, AddressingMode::None);
        opcode_table[0x78] = Opcode::new(0x78, "SEI", 1, 2, AddressingMode::None);

        opcode_table[0x85] = Opcode::new(0x85, "STA", 2, 3, AddressingMode::ZeroPage);
        opcode_table[0x95] = Opcode::new(0x95, "STA", 2, 4, AddressingMode::ZeroPage_X);
        opcode_table[0x8D] = Opcode::new(0x8D, "STA", 3, 4, AddressingMode::Absolute);
        opcode_table[0x9D] = Opcode::new(0x9D, "STA", 3, 5, AddressingMode::Absolute_X);
        opcode_table[0x99] = Opcode::new(0x99, "STA", 3, 5, AddressingMode::Absolute_Y);
        opcode_table[0x81] = Opcode::new(0x81, "STA", 2, 6, AddressingMode::Indirect_X);
        opcode_table[0x91] = Opcode::new(0x91, "STA", 2, 6, AddressingMode::Indirect_Y);

        opcode_table[0x86] = Opcode::new(0x86, "STX", 2, 3, AddressingMode::ZeroPage);
        opcode_table[0x96] = Opcode::new(0x96, "STX", 2, 4, AddressingMode::ZeroPage_X);
        opcode_table[0x8E] = Opcode::new(0x8E, "STX", 3, 4, AddressingMode::Absolute);

        opcode_table[0x84] = Opcode::new(0x84, "STY", 2, 3, AddressingMode::ZeroPage);
        opcode_table[0x94] = Opcode::new(0x94, "STY", 2, 4, AddressingMode::ZeroPage_X);
        opcode_table[0x8C] = Opcode::new(0x8C, "STY", 3, 4, AddressingMode::Absolute);

        opcode_table[0xAA] = Opcode::new(0xAA, "TAX", 1, 2, AddressingMode::None);
        opcode_table[0xA8] = Opcode::new(0xA8, "TAY", 1, 2, AddressingMode::None);
        opcode_table[0xBA] = Opcode::new(0xBA, "TSX", 1, 2, AddressingMode::None);
        opcode_table[0x8A] = Opcode::new(0x8A, "TXA", 1, 2, AddressingMode::None);
        opcode_table[0x9A] = Opcode::new(0x9A, "TXS", 1, 2, AddressingMode::None);
        opcode_table[0x98] = Opcode::new(0x98, "TYA", 1, 2, AddressingMode::None);

        opcode_table
    }
}
