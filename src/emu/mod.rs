pub mod arithmetic;
pub mod branch;
pub mod control;
pub mod data_transfer;
pub mod logical;

use std::{fs::File, io::Read, path::Path};

use crate::parsers::{
    condition::Condition,
    parse_instruction,
    register::{Register, RegisterPair},
    Instruction,
};

const MEMORY_SIZE: usize = 65536;

/**
    Concatenates two expressions of type `u8` into a single value of type `u16`.
    The first parameter is the 8 higher order bits of the resulting `u16` value.
    - `(10101010, 11111111) -> 1010101011111111`
*/
#[macro_export]
macro_rules! join_u8 {
    ($high:expr, $low:expr) => {
        (($high as u16) << 8) | ($low as u16)
    };
}

/**
    Splits an expression of type `u16` into a tuple of two `u8` values. The
    first returned value is the 8 higher order bits of the original `u16`
    expression.
    - `1010101011111111 -> (10101010, 11111111)`
*/
#[macro_export]
macro_rules! split_u16 {
    ($value:expr) => {{
        let value: u16 = $value;
        ((value >> 8) as u8, value as u8)
    }};
}

/**
    Splits an expression of type `u8` into a tuple of two `u8` values. The first
    returned value is the 4 higher order bits of the original `u8` expression.
    - `10101111 -> (00001010, 00001111)`
*/
#[macro_export]
macro_rules! split_u8 {
    ($value:expr) => {{
        let value: u8 = $value;
        (value >> 4, value & 0b00001111)
    }};
}

/**
    The 8080's six 16-bit registers.
*/
#[derive(Debug)]
pub struct Registers {
    pub pc: u16,
    pub sp: u16,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub w: u8,
    pub z: u8,
}

impl Default for Registers {
    fn default() -> Registers {
        Registers {
            pc: 0,
            sp: (MEMORY_SIZE - 1) as u16,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            w: 0,
            z: 0,
        }
    }
}

/**
    The 8080's five control (AKA status) bits.
    - `zero`: Set when an instruction's result is zero.
    - `carry`: Set when an instruction' results in a carry-out.
    - `sign`: Set when an instruction's result is negative.
    - `parity`: Set when an instruction's resulting binary value has an even
       number of ones.
    - `auxiliary_carry`: Set when an instruction results in a carry-out of bit
       three.
*/
#[derive(Debug, Default)]
pub struct Flags {
    pub zero: bool,
    pub carry: bool,
    pub sign: bool,
    pub parity: bool,
    pub auxiliary_carry: bool,
}

/**
    The 8080's arithmetic logic unit (ALU).
*/
#[derive(Debug, Default)]
pub struct ArithmeticLogicUnit {
    pub accumulator: u8,
    pub temporary_accumulator: u8,
    pub flags: Flags,
    pub temporary_register: u8,
}

/**
   The internal state of the 8080.
*/
pub struct State {
    pub registers: Registers,
    pub alu: ArithmeticLogicUnit,
    pub memory: [u8; MEMORY_SIZE],
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("\
            ┌────────────┬────────────┬──────────────────────────────┐\n\
            │ PC: {:#06x} │ SP: {:#06x} │ Accumulator: {:#010b}      │\n\
            ├────────────┴────────────┴┬─────────────────────────────┤\n\
            │ BC: 0b_{:08b}_{:08b} │ DE: 0b_{:08b}_{:08b}    │\n\
            │ HL: 0b_{:08b}_{:08b} │ WZ: 0b_{:08b}_{:08b}    │\n\
            ├──────────┬───────────┬───┴──────┬──────────┬───────────┤\n\
            │ Z: {:05} │ CY: {:05} │ S: {:05} │ P: {:05} │ AC: {:05} │\n\
            ├──────────┴───────────┴──────────┴──────────┴───────────┤\n\
            │ 8-byte Memory Look Ahead: [ {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} ]  │ \n\
            └────────────────────────────────────────────────────────┘",
            self.registers.pc, self.registers.sp, self.alu.accumulator,
            self.registers.b, self.registers.c,
            self.registers.d, self.registers.e,
            self.registers.h, self.registers.l,
            self.registers.w, self.registers.z,
            self.alu.flags.zero,
            self.alu.flags.carry,
            self.alu.flags.sign,
            self.alu.flags.parity,
            self.alu.flags.auxiliary_carry,
            self.get_memory(self.registers.pc),
            self.get_memory(self.registers.pc + 1),
            self.get_memory(self.registers.pc + 2),
            self.get_memory(self.registers.pc + 3),
            self.get_memory(self.registers.pc + 4),
            self.get_memory(self.registers.pc + 5),
            self.get_memory(self.registers.pc + 6),
            self.get_memory(self.registers.pc + 7)
        )).finish()
    }
}

impl Default for State {
    fn default() -> State {
        State {
            registers: Default::default(),
            alu: Default::default(),
            memory: [0; MEMORY_SIZE],
        }
    }
}

impl State {
    pub fn get_register(&self, r: &Register) -> u8 {
        match r {
            Register::A => self.alu.accumulator,
            Register::B => self.registers.b,
            Register::C => self.registers.c,
            Register::D => self.registers.d,
            Register::E => self.registers.e,
            Register::H => self.registers.h,
            Register::L => self.registers.l,
        }
    }

    pub fn set_register(&mut self, r: &Register, data: u8) {
        match r {
            Register::A => self.alu.accumulator = data,
            Register::B => self.registers.b = data,
            Register::C => self.registers.c = data,
            Register::D => self.registers.d = data,
            Register::E => self.registers.e = data,
            Register::H => self.registers.h = data,
            Register::L => self.registers.l = data,
        }
    }

    pub fn get_register_pair(&self, rp: &RegisterPair) -> u16 {
        match rp {
            RegisterPair::BC => join_u8!(self.registers.b, self.registers.c),
            RegisterPair::DE => join_u8!(self.registers.d, self.registers.e),
            RegisterPair::HL => join_u8!(self.registers.h, self.registers.l),
            RegisterPair::SP => self.registers.sp,
        }
    }

    pub fn set_register_pair(&mut self, rp: &RegisterPair, high_data: u8, low_data: u8) {
        match rp {
            RegisterPair::BC => {
                self.registers.b = high_data;
                self.registers.c = low_data;
            }
            RegisterPair::DE => {
                self.registers.d = high_data;
                self.registers.e = low_data;
            }
            RegisterPair::HL => {
                self.registers.h = high_data;
                self.registers.l = low_data;
            }
            RegisterPair::SP => self.registers.sp = join_u8!(high_data, low_data),
        }
    }

    pub fn set_pc(&mut self, address: u16) {
        self.registers.pc = address;
    }

    pub fn get_memory(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn set_memory(&mut self, address: u16, data: u8) {
        self.memory[address as usize] = data;
    }

    pub fn check_condition(&mut self, condition: Condition) -> bool {
        match condition {
            Condition::NZ => !self.alu.flags.zero,
            Condition::Z => self.alu.flags.zero,
            Condition::NC => !self.alu.flags.carry,
            Condition::C => self.alu.flags.carry,
            Condition::PO => !self.alu.flags.parity,
            Condition::PE => self.alu.flags.parity,
            Condition::P => !self.alu.flags.sign,
            Condition::M => self.alu.flags.sign,
        }
    }

    pub fn load_program(&mut self, starting_addr: u16, path_to_program: &str) {
        let mut buffer = Vec::new();
        let _ = match File::open(Path::new(path_to_program)) {
            Ok(mut file) => file.read_to_end(&mut buffer),
            Err(e) => panic!("{}", e.to_string()),
        };

        let address_size = self.memory.len() - starting_addr as usize;
        if address_size < buffer.len() {
            panic!("The program is too large to load into memory.")
        }

        for (index, data) in buffer.into_iter().enumerate() {
            let address = starting_addr + index as u16;
            self.set_memory(address, data);
        }
        self.registers.pc = starting_addr;
    }

    /**
        Fetches the next instruction from memory using the program counter, then
        advances the program counter to the following instruction.
    */
    pub fn fetch_instruction(&mut self) -> Instruction {
        // Instructions are a maximum size of three bytes, so we always read the
        // next three bytes from memory as our input.
        let pc = self.registers.pc as usize;
        let input = (&self.memory[pc..pc + 3], 0usize);

        // Parse the three bytes from the input.
        let (input, instruction) = match parse_instruction(input) {
            Ok((input, instruction)) => (input, instruction),
            Err(e) => panic!("{}", e),
        };

        // Advance program counter depending on the parsed instruction.
        self.registers.pc = (pc as i16 - (input.0.len() as i16 - 3)) as u16;
        // Return the parsed instruction.
        instruction
    }

    pub fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Arithmetic(arithmetic) => arithmetic::execute_arithmetic(self, arithmetic),
            Instruction::Branch(branch) => branch::execute_branch(self, branch),
            Instruction::Control(control) => control::execute_control(self, control),
            Instruction::DataTransfer(data_transfer) => {
                data_transfer::execute_data_transfer(self, data_transfer)
            }
            Instruction::Logical(logical) => logical::execute_logical(self, logical),
        }
    }

    pub fn start(&mut self) {
        let mut instruction_count = 0;
        while usize::from(self.registers.pc) < self.memory.len() {
            println!(
                "{:═^58}",
                format!(" Instruction Number: {instruction_count} ")
            );
            println!("{self:#?}");
            let instruction = self.fetch_instruction();
            println!("{instruction:#?}\n");
            self.execute_instruction(instruction);
            instruction_count += 1;
        }
    }
}
