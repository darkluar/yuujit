use std::fmt;

use crate::bits::{Bit, Bits};

#[derive(Clone, Copy)]
pub enum ArmInstructionType {
    BranchExchange,
    BranchLinkExchangeRegister,
    BranchLinkMaybeExchange,
    DataProcessing,
    Illegal,
}

pub enum Condition {
    Eq,
    Ne,
    Cs,
    Cc,
    Mi,
    Pl,
    Vs,
    Vc,
    Hi,
    Ls,
    Ge,
    Lt,
    Gt,
    Le,
    Al,
    Nv,
}

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Condition::Eq => todo!(),
            Condition::Ne => todo!(),
            Condition::Cs => todo!(),
            Condition::Cc => todo!(),
            Condition::Mi => todo!(),
            Condition::Pl => todo!(),
            Condition::Vs => todo!(),
            Condition::Vc => todo!(),
            Condition::Hi => todo!(),
            Condition::Ls => todo!(),
            Condition::Ge => todo!(),
            Condition::Lt => todo!(),
            Condition::Gt => todo!(),
            Condition::Le => todo!(),
            Condition::Al => write!(f, ""),
            Condition::Nv => todo!(),
        }
    }
}

impl From<u32> for Condition {
    fn from(value: u32) -> Self {
        match value {
            0 => Condition::Eq,
            1 => Condition::Ne,
            2 => Condition::Cs,
            3 => Condition::Cc,
            4 => Condition::Mi,
            5 => Condition::Pl,
            6 => Condition::Vs,
            7 => Condition::Vc,
            8 => Condition::Hi,
            9 => Condition::Ls,
            10 => Condition::Ge,
            11 => Condition::Lt,
            12 => Condition::Gt,
            13 => Condition::Le,
            14 => Condition::Al,
            15 => Condition::Nv,
            _ => unimplemented!(),
        }
    }
}

pub enum ShiftType {
    Lsl,
    Lsr,
    Asr,
    Ror,
}

impl From<u32> for ShiftType {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Lsl,
            1 => Self::Lsr,
            2 => Self::Asr,
            3 => Self::Ror,
            _ => unimplemented!(),
        }
    }
}

// Used in data processing.
pub enum ShiftedOperand {
    Imm(ImmShiftedOperand),
    Reg(RegShiftedOperand),
}

impl From<u32> for ShiftedOperand {
    fn from(value: u32) -> Self {
        if value.bit(25) {
            Self::Imm(ImmShiftedOperand::from(value))
        } else {
            Self::Reg(RegShiftedOperand::from(value))
        }
    }
}

pub struct ImmShiftedOperand {
    pub shift: u32,

    // TODO: Create an Imm struct to assert at runtime that the immediate
    // fits within n bits.
    pub rotated: u32,
}

impl From<u32> for ImmShiftedOperand {
    fn from(value: u32) -> Self {
        let shift = value.bits(8, 4) * 2;
        Self {
            shift,
            rotated: value.bits(0, 8).rotate_right(shift),
        }
    }
}

pub enum ShiftAmount {
    Imm(u8),
    Reg(u8),
}

impl From<u32> for ShiftAmount {
    fn from(value: u32) -> Self {
        if value.bit(4) {
            Self::Reg(value.bits(8, 4) as u8)
        } else {
            Self::Imm(value.bits(7, 5) as u8)
        }
    }
}

pub struct RegShiftedOperand {
    rm: u8,
    shift_type: ShiftType,
    shift_amount: ShiftAmount,
}

impl From<u32> for RegShiftedOperand {
    fn from(value: u32) -> Self {
        Self {
            rm: value.bits(0, 4) as u8,
            shift_type: ShiftType::from(value),
            shift_amount: ShiftAmount::from(value),
        }
    }
}

#[derive(Debug)]
pub enum DataProcessingOpcode {
    And,
    Eor,
    Sub,
    Rsb,
    Add,
    Adc,
    Sbc,
    Rsc,
    Tst,
    Teq,
    Cmp,
    Cmn,
    Orr,
    Mov,
    Bic,
    Mvn,
}

impl DataProcessingOpcode {
    pub fn uses_destination(&self) -> bool {
        !matches!(self, Self::Tst | Self::Teq | Self::Cmp | Self::Cmn)
    }

    pub fn uses_carry(&self) -> bool {
        matches!(self, Self::Adc | Self::Sbc | Self::Rsc)
    }

    pub fn uses_lhs(&self) -> bool {
        !matches!(self, Self::Mov | Self::Mvn)
    }
}

impl From<u32> for DataProcessingOpcode {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::And,
            1 => Self::Eor,
            2 => Self::Sub,
            3 => Self::Rsb,
            4 => Self::Add,
            5 => Self::Adc,
            6 => Self::Sbc,
            7 => Self::Rsc,
            8 => Self::Tst,
            9 => Self::Teq,
            10 => Self::Cmp,
            11 => Self::Cmn,
            12 => Self::Orr,
            13 => Self::Mov,
            14 => Self::Bic,
            15 => Self::Mvn,
            _ => unreachable!(),
        }
    }
}

pub struct DataProcessing {
    pub rd: u8,
    pub rn: u8,
    pub set_flags: bool,
    pub opcode: DataProcessingOpcode,
    pub operand: ShiftedOperand,
    pub condition: Condition,
}

impl From<u32> for DataProcessing {
    fn from(value: u32) -> Self {
        Self {
            rd: value.bits(12, 4) as u8,
            rn: value.bits(16, 4) as u8,
            set_flags: value.bit(20),
            opcode: DataProcessingOpcode::from(value.bits(21, 4)),
            operand: ShiftedOperand::from(value),
            condition: Condition::from(value.bits(28, 4)),
        }
    }
}