use crate::arch::board::LD_STACK_PTR;
use crate::arch::vector::{Vector, VectorTable};
use crate::{const_vec, irq_default, main, vector};

pub const ISR_TABLE_SIZ: usize = 2;

// Configurable Fault Status Register (MMFSR, BFSR, UFSR)
pub const CFSR_ADDR: usize = 0xE000_ED28;

#[derive(Clone, Copy)]
pub enum IsrKind {
    Undef,
    Stp,
    Reset,
}

// Original Vector Table
pub static ISR_TABLE: VectorTable<Vector, ISR_TABLE_SIZ> = const_vec!(
    ISR_TABLE_SIZ,
    vector!(null),
    vector!(IsrKind::Stp, extern LD_STACK_PTR),
    // Thumb-mode, least-significant bit will be set
    vector!(IsrKind::Reset, fn main)
);
