use super::register::Register;

pub mod ana;
pub mod ani;
pub mod cma;
pub mod cmc;
pub mod cmp;
pub mod cpi;
pub mod ora;
pub mod ori;
pub mod ral;
pub mod rar;
pub mod rlc;
pub mod rrc;
pub mod stc;
pub mod xra;
pub mod xri;

pub struct ANDRegister {
    pub r: Register,
}

pub struct ANDMemory {}

pub struct ANDImmediate {
    pub data: u8,
}

pub struct ExclusiveORRegister {
    pub r: Register,
}

pub struct ExclusiveORMemory {}

pub struct ExclusiveORImmediate {
    pub data: u8,
}

pub struct ORRegister {
    pub r: Register,
}

pub struct ORMemory {}

pub struct ORImmediate {
    pub data: u8,
}

pub struct CompareRegister {
    pub r: Register,
}

pub struct CompareMemory {}

pub struct CompareImmediate {
    pub data: u8,
}

pub struct RotateLeft {}

pub struct RotateRight {}

pub struct RotateLeftThroughCarry {}

pub struct RotateRightThroughCarry {}

pub struct ComplementAccumulator {}

pub struct ComplementCarry {}

pub struct SetCarry {}
