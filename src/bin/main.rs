#![no_std]
#![no_main]
#![feature(naked_functions)]
#![allow(named_asm_labels)]
#![allow(undefined_naked_function_abi)]

use asm::ptr::write_slice;
use core::arch::{naked_asm};
use asm::{const_vec, vector};
use asm::vector::{Vector, VectorTable};

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern {
    static LD_STACK_PTR: usize;
}

// Original Vector Table
static ISR_TABLE : VectorTable<Vector, 3> = const_vec!(3, vector!(null),
    vector!("STP", extern LD_STACK_PTR),
    // Thumb-mode, least-significant bit will be set
    vector!("Reset", fn main)
);

// Cortex-M4 Vector Table we are going to bake into the binary
#[link_section = ".vector_table"]
#[no_mangle]
static VECTOR_TABLE : VectorTable<*const (), 3> = ISR_TABLE.raw();

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


#[no_mangle]
pub fn main() {
    write_slice(0x01 as _, b"Welcome");
}

#[no_mangle]
pub fn busy() -> ! {
    loop {}
}

