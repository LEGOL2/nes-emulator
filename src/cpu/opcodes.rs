use super::{AddressingMode, Status, CPU};

impl CPU {
    pub fn interpret(&mut self, opcode: u8) -> bool {
        match opcode {
            0x00 => {
                return false;
            }

            0xA9 => {
                self.lda(AddressingMode::Immediate);
                self.program_counter += 1;
            }
            0xA5 => {
                self.lda(AddressingMode::ZeroPage);
                self.program_counter += 1;
            }
            0xB5 => {
                self.lda(AddressingMode::ZeroPage_X);
                self.program_counter += 1;
            }
            0xAD => {
                self.lda(AddressingMode::Absolute);
                self.program_counter += 1;
            }
            0xBD => {
                self.lda(AddressingMode::Absolute_X);
                self.program_counter += 1;
            }
            0xB9 => {
                self.lda(AddressingMode::Absolute_Y);
                self.program_counter += 1;
            }
            0xA1 => {
                self.lda(AddressingMode::Indirect_X);
                self.program_counter += 1;
            }
            0xB1 => {
                self.lda(AddressingMode::Indirect_Y);
                self.program_counter += 1;
            }

            // ADC
            0x69 => {
                self.adc(AddressingMode::Immediate);
                self.program_counter += 1;
            }
            0x65 => {
                self.adc(AddressingMode::ZeroPage);
                self.program_counter += 1;
            }
            0x75 => {
                self.adc(AddressingMode::ZeroPage_X);
                self.program_counter += 1;
            }
            0x6D => {
                self.adc(AddressingMode::Absolute);
                self.program_counter += 1;
            }
            0x7D => {
                self.adc(AddressingMode::Absolute_X);
                self.program_counter += 1;
            }
            0x79 => {
                self.adc(AddressingMode::Absolute_Y);
                self.program_counter += 1;
            }
            0x61 => {
                self.adc(AddressingMode::Indirect_X);
                self.program_counter += 1;
            }
            0x71 => {
                self.adc(AddressingMode::Indirect_Y);
                self.program_counter += 1;
            }

            0xAA => self.tax(),

            0xE8 => self.inx(),

            _ => todo!(""),
        }

        true
    }

    fn adc(&mut self, mode: AddressingMode) {
        let address = self.get_operand_address(mode);
        let value = self.mem_read(address);
        self.add_to_accumulator(value);
        self.update_zero_and_negative_flags(self.accumulator);
    }

    fn lda(&mut self, mode: AddressingMode) {
        let address = self.get_operand_address(mode);
        let value = self.mem_read(address);

        self.accumulator = value;
        self.update_zero_and_negative_flags(self.accumulator);
    }

    fn tax(&mut self) {
        self.register_x = self.accumulator;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn inx(&mut self) {
        if self.register_x == std::u8::MAX {
            self.register_x = 0;
        } else {
            self.register_x += 1;
        }
        self.update_zero_and_negative_flags(self.register_x);
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
}
