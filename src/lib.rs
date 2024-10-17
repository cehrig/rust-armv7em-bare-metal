#![no_std]

use core::arch::asm;

pub mod arch;
pub mod ptr;

use crate::arch::cpu::cortex_m4::AIRCR_ADDR;
use crate::ptr::{read_volatile, write_volatile};
pub use arch::board::*;

// Dummy placeholders
#[no_mangle]
pub unsafe fn main() -> ! {
    // RESET
    let bla = AIRCR_ADDR as *mut u32;
    write_volatile(bla, [0x05FA_0004]);

    loop {}
}

#[no_mangle]
pub unsafe fn irq_default() -> usize {
    loop {}
}

#[no_mangle]
pub unsafe fn fault() -> usize {
    /*
    let mut ipsr: usize;

    asm!(
    "MRS {ipsr}, IPSR",
    ipsr = out(reg) ipsr,
    );

    let irq = ISR_TABLE.get(ipsr);

    match irq.kind() {
        IsrKind::HardFault => {}
        IsrKind::BusFault => {}
        IsrKind::MmuFault => {}
        IsrKind::UsageFault => {}
        _ => {
            //unreachable!("")
        }
    };

    let mut cfsr: usize;

    /*
    asm!(
    "LDR r0, ={cfsr_addr}",
    "LDR {cfsr}, [r0]",
    cfsr_addr = const CFSR_ADDR,
    cfsr = out(reg) cfsr
    );

     */

     */

    loop {}
}
