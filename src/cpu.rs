mod opcodes;

#[cfg(test)]
mod cpu_tests;

pub struct CPU<'a> {
    pub accumulator: u8,
    pub status: Status,
    pub program_counter: u16,
    pub register_x: u8,
    pub register_y: u8,
    memory: [u8; 0xFFFF],
    opcode_table: [opcodes::Opcode<'a>; 0xFF],
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPage_X,
    Absolute,
    Absolute_X,
    Absolute_Y,
    Indirect_X,
    Indirect_Y,
    None,
}

/// # Status Register (P) http://wiki.nesdev.com/w/index.php/Status_flags
///
///  7 6 5 4 3 2 1 0
///  N V _ B D I Z C
///  | |   | | | | +--- Carry Flag
///  | |   | | | +----- Zero Flag
///  | |   | | +------- Interrupt Disable
///  | |   | +--------- Decimal Mode (not used on NES)
///  | |   +----------- Break Command
///  | +--------------- Overflow Flag
///  +----------------- Negative Flag
///
pub struct Status {
    status: u8,
}
impl Status {
    const CARRY: u8 = 0b0000_0001;
    const ZERO: u8 = 0b0000_0010;
    const INTERRUPT_DISABLE: u8 = 0b0000_0100;
    #[allow(dead_code)]
    const DECIMAL_MODE: u8 = 0b0000_1000;
    const BREAK: u8 = 0b0001_0000;
    const BREAK2: u8 = 0b0010_0000;
    const OVERFLOW: u8 = 0b0100_0000;
    const NEGATIV: u8 = 0b1000_0000;

    fn set(&mut self, flag: u8) {
        self.status |= flag;
    }

    fn reset(&mut self, flag: u8) {
        self.status &= !flag;
    }

    fn get(&self) -> u8 {
        self.status
    }
}

impl<'a> CPU<'a> {
    pub fn new() -> Self {
        let opcodes = CPU::create_opcode_table();

        CPU {
            accumulator: 0,
            status: Status { status: 0 },
            program_counter: 0,
            register_x: 0,
            register_y: 0,
            memory: [0; 0xFFFF],
            opcode_table: opcodes,
        }
    }

    fn mem_read(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    fn mem_read_u16(&self, position: u16) -> u16 {
        let lo = self.mem_read(position) as u16;
        let hi = self.mem_read(position + 1) as u16;

        (hi << 8) | lo
    }

    fn mem_write(&mut self, address: u16, data: u8) {
        self.memory[address as usize] = data;
    }

    fn mem_write_u16(&mut self, address: u16, data: u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & 0xFF) as u8;
        self.mem_write(address, lo);
        self.mem_write(address + 1, hi);
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(&program[..]);
        self.mem_write_u16(0xFFFC, 0x8000);
    }

    pub fn reset(&mut self) {
        self.accumulator = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.status.reset(0xff);

        self.program_counter = self.mem_read_u16(0xFFFC);
    }

    pub fn run(&mut self) {
        let mut continue_execution = true;
        while continue_execution {
            let opcode_number = self.mem_read(self.program_counter);
            let opcode = self.opcode_table[opcode_number as usize];
            self.program_counter += 1;

            continue_execution = self.interpret(&opcode);
        }
    }

    fn get_operand_address(&self, mode: AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.program_counter,
            AddressingMode::ZeroPage => self.mem_read(self.program_counter) as u16,
            AddressingMode::ZeroPage_X => {
                let position = self.mem_read(self.program_counter);
                let address = position.wrapping_add(self.register_x) as u16;
                address
            }
            AddressingMode::Absolute => self.mem_read_u16(self.program_counter),
            AddressingMode::Absolute_X => {
                let base = self.mem_read_u16(self.program_counter);
                let address = base.wrapping_add(self.register_x as u16);
                address
            }
            AddressingMode::Absolute_Y => {
                let base = self.mem_read_u16(self.program_counter);
                let address = base.wrapping_add(self.register_y as u16);
                address
            }
            AddressingMode::Indirect_X => {
                let base = self.mem_read(self.program_counter);

                let ptr = (base as u8).wrapping_add(self.register_x);
                let lo = self.mem_read(ptr as u16);
                let hi = self.mem_read(ptr.wrapping_add(1) as u16);
                (hi as u16) << 8 | (lo as u16)
            }
            AddressingMode::Indirect_Y => {
                let base = self.mem_read(self.program_counter);

                let lo = self.mem_read(base as u16);
                let hi = self.mem_read((base as u8).wrapping_add(1) as u16);
                let deref_base = (hi as u16) << 8 | (lo as u16);
                let deref = deref_base.wrapping_add(self.register_y as u16);
                deref
            }
            AddressingMode::None => {
                panic!("Wrong addressing mode!");
            }
        }
    }

    fn increment_program_counter(&mut self, step: u8) {
        self.program_counter += step as u16 - 1;
    }
}
