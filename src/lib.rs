#![no_std]

pub mod arch;
pub mod ptr;

pub use arch::board::*;

use crate::arch::register::RegisterOps;

// Dummy placeholders
#[no_mangle]
pub unsafe fn main() -> ! {
    RCC_AHB1ENR.set(GPIOB);
    RCC_AHB1ENR.set(GPIOC);

    GPIOB_MODER.set_from(GPIO_MODER_PORT_7, 1);

    loop {
        let pressed = GPIOC_IDR.all_set(GPIO_IDR_PORT_13);

        if pressed {
            GPIOB_ODR.set(GPIO_ODR_PORT_7);
        } else {
            GPIOB_ODR.clear(GPIO_ODR_PORT_7);
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
