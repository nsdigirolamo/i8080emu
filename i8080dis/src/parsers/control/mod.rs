use super::register::RegisterPair;

pub mod di;
pub mod ei;
pub mod hlt;
pub mod input;
pub mod nop;
pub mod out;
pub mod pop;
pub mod push;
pub mod sphl;
pub mod xthl;

pub struct Push {
    pub rp: RegisterPair,
}

pub struct PushProcessorStatusWord {}

pub struct Pop {
    pub rp: RegisterPair,
}

pub struct PopProcessorStatusWord {}

pub struct ExchangeStackTopWithHL {}

pub struct MoveHLtoSP {}

pub struct Input {
    pub port: u8,
}

pub struct Output {
    pub port: u8,
}

pub struct EnableInterrupts {}

pub struct DisableInterrupts {}

pub struct Halt {}

pub struct NoOp {}
