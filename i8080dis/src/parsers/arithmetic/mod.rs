use super::register_parsers::{Register, RegisterPair};

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
    r: Register,
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
    rp: RegisterPair,
}

#[derive(Debug, PartialEq)]
pub struct DecrementRegisterPair {
    rp: RegisterPair,
}

#[derive(Debug, PartialEq)]
pub struct AddRegisterPairToHL {
    rp: RegisterPair,
}

#[derive(Debug, PartialEq)]
pub struct DecimalAdjustAccumulator {}
