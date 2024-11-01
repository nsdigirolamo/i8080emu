#![allow(dead_code)]

use super::register_parsers::{Register, RegisterPair};

pub mod mov;
pub mod mvi;

const MOV_OPCODE: &str = "01";
const MVI_OPCODE: &str = "00";

#[derive(Debug)]
pub struct MoveRegister {
    to_register: Register,
    from_register: Register,
}

#[derive(Debug)]
pub struct MoveFromMemory {
    to_register: Register,
}

#[derive(Debug)]
pub struct MoveToMemory {
    from_register: Register,
}

#[derive(Debug)]
pub struct MoveImmediate {
    to_register: Register,
    from_data: u8,
}

pub struct MoveToMemoryImmediate {
    from_data: u8
}

pub struct LoadRegisterPairImmediate {
    to_register_pair: RegisterPair,
    from_data: u16
}

pub struct LoadAccumulatorDirect {
    from_address: u16
}

pub struct StoreAccumulatorDirect {
    to_address: u16
}

pub struct LoadHLDirect {
    from_address: u16
}

pub struct StoreHLDirect {
    to_address: u16
}

pub struct LoadAccumulatorIndirect {

}