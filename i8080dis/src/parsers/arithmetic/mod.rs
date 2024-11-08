use super::register::{Register, RegisterPair};

pub mod aci;
pub mod adc;
pub mod add;
pub mod adi;
pub mod daa;
pub mod dad;
pub mod dcr;
pub mod dcx;
pub mod inr;
pub mod inx;
pub mod sbb;
pub mod sbi;
pub mod sub;
pub mod sui;

#[derive(Debug, PartialEq)]
pub struct AddRegister {
    pub r: Register,
}

#[derive(Debug, PartialEq)]
pub struct AddMemory {}

#[derive(Debug, PartialEq)]
pub struct AddImmediate {
    pub data: u8,
}

#[derive(Debug, PartialEq)]
pub struct AddRegisterWithCarry {
    pub r: Register,
}

#[derive(Debug, PartialEq)]
pub struct AddMemoryWithCarry {}

#[derive(Debug, PartialEq)]
pub struct AddImmediateWithCarry {
    pub data: u8,
}

#[derive(Debug, PartialEq)]
pub struct SubtractRegister {
    pub r: Register,
}

#[derive(Debug, PartialEq)]
pub struct SubtractMemory {}

#[derive(Debug, PartialEq)]
pub struct SubtractImmediate {
    pub data: u8,
}

#[derive(Debug, PartialEq)]
pub struct SubtractRegisterWithBorrow {
    pub r: Register,
}

#[derive(Debug, PartialEq)]
pub struct SubtractMemoryWithBorrow {}

#[derive(Debug, PartialEq)]
pub struct SubtractImmediateWithBorrow {
    pub data: u8,
}

#[derive(Debug, PartialEq)]
pub struct IncrementRegister {
    pub r: Register,
}

#[derive(Debug, PartialEq)]
pub struct IncrementMemory {}

#[derive(Debug, PartialEq)]
pub struct DecrementRegister {
    pub r: Register,
}

#[derive(Debug, PartialEq)]
pub struct DecrementMemory {}

#[derive(Debug, PartialEq)]
pub struct IncrementRegisterPair {
    pub rp: RegisterPair,
}

#[derive(Debug, PartialEq)]
pub struct DecrementRegisterPair {
    pub rp: RegisterPair,
}

#[derive(Debug, PartialEq)]
pub struct AddRegisterPairToHL {
    pub rp: RegisterPair,
}

#[derive(Debug, PartialEq)]
pub struct DecimalAdjustAccumulator {}
