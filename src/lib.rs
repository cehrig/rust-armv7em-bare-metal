#![no_std]

use core::arch::asm;

pub mod arch;
pub mod ptr;

pub use arch::board::*;

// Dummy placeholders
#[no_mangle]
pub unsafe fn main() -> ! {
    loop {}
}

#[no_mangle]
pub unsafe fn irq_default() -> usize {
    let mut ipsr: usize;
    let mut cfsr: usize;

    asm!(
        "MRS {ipsr}, IPSR",
        "LDR r2, =0xE000ED28",
        "LDR {cfsr}, [r2]",
        ipsr = out(reg) ipsr,
        cfsr = out(reg) cfsr
    );

    loop {}
}
