pub mod data_transfer;

use data_transfer::execute_data_transfer;

use crate::parsers::{
    register::{Register, RegisterPair},
    Instruction,
};

/**
  Concatenates two expressions of type `u8` into a single expression of type
  `u16`: First parameter is the higher order `u8` expression.
  - `(10101010, 11111111) -> 1010101011111111`
*/
#[macro_export]
macro_rules! concat_u8_pair {
    ($high:expr, $low:expr) => {
        (($high as u16) << 8) | ($low as u16)
    };
}

/**
  Splits an expression of type `u16` into a tuple of two `u8` expressions. The
  first returned expression is the higher order `u8` result.
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
  Splits an expression of type `u8` into a tuple of two `u8` expressions. The
  first returned expression is the
  - `10101111 -> (00001010, 00001111)`
*/
#[macro_export]
macro_rules! split_u8 {
    ($value:expr) => {{
        let value: u8 = $value;
        (value >> 4, value & 0b00001111)
    }};
}

/** The 8080's six 16-bit registers. */
struct Registers {
    pc: u16, // Program Counter
    sp: u16, // Stack Pointer
    b: u8,   // BC Register
    c: u8,
    d: u8, // DE Register
    e: u8,
    h: u8, // HL Register
    l: u8,
    w: u8, // WZ Temporary Register
    z: u8,
}

/** The 8080's five control (AKA status) bits. */
struct Flags {
    zero: bool,
    carry: bool,
    sign: bool,
    parity: bool,
    auxiliary_carry: bool,
}

struct ArithmeticLogicUnit {
    accumulator: u8,
    temporary_accumulator: u8,
    flags: Flags,
    temporary_register: u8,
}

pub struct State {
    registers: Registers,
    alu: ArithmeticLogicUnit,
    memory: [u8; 65536],
}

impl State {
    fn get_register(&self, r: &Register) -> u8 {
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

    fn set_register(&mut self, r: &Register, data: u8) {
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

    fn get_register_pair(&self, rp: &RegisterPair) -> u16 {
        match rp {
            RegisterPair::BC => concat_u8_pair!(self.registers.b, self.registers.c),
            RegisterPair::DE => concat_u8_pair!(self.registers.d, self.registers.e),
            RegisterPair::HL => concat_u8_pair!(self.registers.h, self.registers.l),
            RegisterPair::SP => self.registers.sp,
        }
    }

    fn set_register_pair(&mut self, rp: &RegisterPair, high_data: u8, low_data: u8) {
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

    fn get_memory(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    fn set_memory(&mut self, address: u16, data: u8) {
        self.memory[address as usize] = data;
    }

    fn set_flags(&mut self, result: i8, carried: bool) {
        self.alu.flags.carry = carried;
        self.alu.flags.zero = result == 0;
    }
}

pub fn execute_instruction(state: &mut State, instruction: Instruction) {
    match instruction {
        Instruction::Arithmetic(_arithmetic) => todo!(),
        Instruction::Branch(_branch) => todo!(),
        Instruction::Control(_control) => todo!(),
        Instruction::DataTransfer(data_transfer) => execute_data_transfer(state, data_transfer),
        Instruction::Logical(_logical) => todo!(),
    }
}
