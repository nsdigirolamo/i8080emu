pub mod arithmetic;
// pub mod branch;
pub mod data_transfer;
pub mod logical;

use std::{fs::File, io::Read, path::Path};

use arithmetic::execute_arithmetic;
use data_transfer::execute_data_transfer;

use crate::parsers::{
    parse_instruction,
    register::{Register, RegisterPair},
    Instruction,
};

const MEMORY_SIZE: u16 = 65535;

/**
    Concatenates two expressions of type `u8` into a single value of type `u16`.
    The first parameter is the 8 higher order bits of the resulting `u16` value.
    - `(10101010, 11111111) -> 1010101011111111`
*/
#[macro_export]
macro_rules! concat_u8_pair {
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
#[derive(Default)]
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
#[derive(Default)]
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
#[derive(Default)]
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
    pub memory: [u8; MEMORY_SIZE as usize],
}

impl Default for State {
    fn default() -> State {
        State {
            registers: Default::default(),
            alu: Default::default(),
            memory: [0; MEMORY_SIZE as usize],
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
            RegisterPair::BC => concat_u8_pair!(self.registers.b, self.registers.c),
            RegisterPair::DE => concat_u8_pair!(self.registers.d, self.registers.e),
            RegisterPair::HL => concat_u8_pair!(self.registers.h, self.registers.l),
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
            RegisterPair::SP => self.registers.sp = concat_u8_pair!(high_data, low_data),
        }
    }

    pub fn get_memory(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn set_memory(&mut self, address: u16, data: u8) {
        self.memory[address as usize] = data;
    }

    pub fn load_program(&mut self, starting_addr: u16, path_to_program: &str) {
        let mut buffer = Vec::new();
        let _ = match File::open(Path::new(path_to_program)) {
            Ok(mut file) => file.read_to_end(&mut buffer),
            Err(e) => panic!("{}", e.to_string()),
        };

        let address_size = (MEMORY_SIZE - starting_addr) as usize;
        if address_size < buffer.len() {
            panic!("The program is too large to load into memory.")
        }

        for (index, data) in buffer.into_iter().enumerate() {
            let address = starting_addr + index as u16;
            self.set_memory(address, data);
        }
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
            Ok((_, instruction)) => (input, instruction),
            Err(e) => panic!("{}", e),
        };

        // Advance program counter depending on the parsed instruction.
        self.registers.pc = pc as u16 - (input.0.len() - 3) as u16;
        // Return the parsed instruction.
        instruction
    }

    pub fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Arithmetic(arithmetic) => execute_arithmetic(self, arithmetic),
            Instruction::Branch(_branch) => todo!(),
            Instruction::Control(_control) => todo!(),
            Instruction::DataTransfer(data_transfer) => execute_data_transfer(self, data_transfer),
            Instruction::Logical(logical) => logical::execute_logical(self, logical),
        }
    }

    pub fn start(&mut self) {
        while self.registers.pc < MEMORY_SIZE {
            let instruction = self.fetch_instruction();
            self.execute_instruction(instruction);
        }
    }
}
