#![no_std]
#![no_main]
#![feature(naked_functions)]
#![allow(named_asm_labels)]
#![allow(undefined_naked_function_abi)]

use asm::arch::vector::VectorTable;
use core::arch::naked_asm;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[cfg(feature = "st32f4xx")]
use asm::arch::board::st32f4xx::*;

// Cortex-M4 Vector Table we are going to bake into the binary
#[link_section = ".vector_table"]
#[no_mangle]
static VECTOR_TABLE: VectorTable<*const (), ISR_TABLE_SIZ> = ISR_TABLE.raw();

#[naked]
#[no_mangle]
pub unsafe fn _start() {
    naked_asm!(
        "ldr     r0, =LD_STACK_PTR",
        "mov     sp, r0",
        "bl      main",
        ".equ PSCI_SYSTEM_OFF, 0x84000008",
        "ldr     r0, =PSCI_SYSTEM_OFF",
        "hvc     #0"
    );
}
