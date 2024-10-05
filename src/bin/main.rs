#![no_std]
#![no_main]
#![feature(naked_functions)]
#![allow(named_asm_labels)]
#![allow(undefined_naked_function_abi)]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

use asm::ptr::write_slice;
use core::arch::asm;

const UART0: *mut u8 = 0x0900_0000 as *mut u8;

#[naked]
#[no_mangle]
pub unsafe fn _start() {
    asm!(
        "ldr     x30, =LD_STACK_PTR",
        "mov     sp, x30",
        "bl      main",
        ".equ PSCI_SYSTEM_OFF, 0x84000008",
        "ldr     x0, =PSCI_SYSTEM_OFF",
        "hvc     #0",
        options(noreturn)
    );
}

#[no_mangle]
pub fn main() {
    write_slice(UART0, b"Welcome");
}
