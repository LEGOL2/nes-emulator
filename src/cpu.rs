#[cfg(test)]
mod cpu_tests;

pub struct CPU {
    pub accumulator: u8,
    pub status: u8,
    pub program_counter: u16,
    pub register_x: u8,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            accumulator: 0,
            status: 0,
            program_counter: 0,
            register_x: 0,
        }
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
        self.program_counter = 0;

        loop {
            let opcode = program[self.program_counter as usize];
            self.program_counter += 1;

            match opcode {
                0x00 => {
                    return;
                }

                0xA9 => {
                    let param = self.fetch_param(&program);
                    self.lda(param);
                }

                0xAA => self.tax(),

                0xE8 => self.inx(),

                _ => todo!("")
            }
        }
    }

    fn fetch_param(&mut self, program: &Vec<u8>) -> u8 {
        let param = program[self.program_counter as usize];
        self.program_counter += 1;

        param
    }

    fn lda(&mut self, value: u8) {
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
            self.status = self.status | 0b0000_0010;
        } else {
            self.status = self.status & 0b1111_1101;
        }

        if result & 0b1000_0000 != 0 {
            self.status = self.status | 0b1000_0000;
        } else {
            self.status = self.status & 0b0111_1111;
        }
    }
}
