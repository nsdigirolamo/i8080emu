use super::register::{Register, RegisterPair};

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

#[derive(Debug, PartialEq)]
pub struct MoveRegister {
    pub r1: Register,
    pub r2: Register,
}

#[derive(Debug, PartialEq)]
pub struct MoveFromMemory {
    pub r: Register,
}

#[derive(Debug, PartialEq)]
pub struct MoveToMemory {
    pub r: Register,
}

#[derive(Debug, PartialEq)]
pub struct MoveImmediate {
    pub r: Register,
    pub data: u8,
}

#[derive(Debug, PartialEq)]
pub struct MoveToMemoryImmediate {
    pub data: u8,
}

#[derive(Debug, PartialEq)]
pub struct LoadRegisterPairImmediate {
    pub rp: RegisterPair,
    pub low_data: u8,
    pub high_data: u8,
}

#[derive(Debug, PartialEq)]
pub struct LoadAccumulatorDirect {
    pub low_addr: u8,
    pub high_addr: u8,
}

#[derive(Debug, PartialEq)]
pub struct StoreAccumulatorDirect {
    pub low_addr: u8,
    pub high_addr: u8,
}

#[derive(Debug, PartialEq)]
pub struct LoadHLDirect {
    pub low_addr: u8,
    pub high_addr: u8,
}

#[derive(Debug, PartialEq)]
pub struct StoreHLDirect {
    pub low_addr: u8,
    pub high_addr: u8,
}

#[derive(Debug, PartialEq)]
pub struct LoadAccumulatorIndirect {
    pub rp: RegisterPair,
}

#[derive(Debug, PartialEq)]
pub struct StoreAccumulatorIndirect {
    pub rp: RegisterPair,
}

#[derive(Debug, PartialEq)]
pub struct ExchangeHLtoDE {}
