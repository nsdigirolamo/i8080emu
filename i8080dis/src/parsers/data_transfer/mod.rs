#![allow(dead_code)]

use super::register_parsers::{Register, RegisterPair};

pub mod lda;
pub mod ldax;
pub mod lhld;
pub mod lxi;
pub mod mov;
pub mod mvi;
pub mod shld;
pub mod sta;
pub mod stax;
pub mod xchg;

#[derive(Debug)]
pub struct MoveRegister {
    r1: Register,
    r2: Register,
}

#[derive(Debug)]
pub struct MoveFromMemory {
    r: Register,
}

#[derive(Debug)]
pub struct MoveToMemory {
    r: Register,
}

#[derive(Debug)]
pub struct MoveImmediate {
    r: Register,
    data: u8,
}

#[derive(Debug)]
pub struct MoveToMemoryImmediate {
    data: u8,
}

#[derive(Debug)]
pub struct LoadRegisterPairImmediate {
    rp: RegisterPair,
    low_data: u8,
    high_data: u8,
}

#[derive(Debug)]
pub struct LoadAccumulatorDirect {
    low_addr: u8,
    high_addr: u8,
}

#[derive(Debug)]
pub struct StoreAccumulatorDirect {
    low_addr: u8,
    high_addr: u8,
}

#[derive(Debug)]
pub struct LoadHLDirect {
    low_addr: u8,
    high_addr: u8,
}

#[derive(Debug)]
pub struct StoreHLDirect {
    low_addr: u8,
    high_addr: u8,
}

#[derive(Debug)]
pub struct LoadAccumulatorIndirect {
    rp: RegisterPair,
}

#[derive(Debug)]
pub struct StoreAccumulatorIndirect {
    rp: RegisterPair,
}

#[derive(Debug)]
pub struct ExchangeHLtoDE {}
