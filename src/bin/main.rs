#![no_std]
#![no_main]
#![feature(naked_functions)]
#![allow(named_asm_labels)]
#![allow(undefined_naked_function_abi)]

use asm::arch::vector::VectorTable;
use asm::{ISR_TABLE, ISR_TABLE_SIZ};

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

// Cortex-M4 Vector Table we are going to bake into the binary
#[link_section = ".vector_table"]
#[no_mangle]
static VECTOR_TABLE: VectorTable<*const (), ISR_TABLE_SIZ> = ISR_TABLE.raw();
