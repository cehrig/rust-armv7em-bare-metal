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
    //let aircr = AIRCR_ADDR as *mut u32;
    //write_volatile(aircr, [0x05FA_0004]);

    // Clock on GPIOB, GPIOC
    write_volatile(RCC_AHB1ENR_ADDR as *mut usize, [0x0000_0006]);

    // Clock on GPIOC
    //write_volatile(RCC_AHB1ENR_ADDR as *mut usize, [0x0000_0004]);

    // PB7 as output
    write_volatile(GPIOB_MODER_ADDR as *mut usize, [0x0000_4000]);

    loop {
        // Button pressed?
        let set: u32 = read_volatile(GPIOC_IDR_ADDR as _);

        if set & 0x0000_2000 > 0 {
            write_volatile(GPIOB_ODR_ADDR as *mut usize, [0x0000_0080]);
        } else {
            write_volatile(GPIOB_ODR_ADDR as *mut usize, [0x0000_0000]);
        }
    }
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

    asm!(
    "LDR r0, ={cfsr_addr}",
    "LDR {cfsr}, [r0]",
    cfsr_addr = const CFSR_ADDR,
    cfsr = out(reg) cfsr
    );

     */

    loop {}
}
