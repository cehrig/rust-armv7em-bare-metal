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
use core::arch::{asm, naked_asm};

extern {
    static LD_STACK_PTR: usize;
}

#[derive(Copy, Clone)]
#[repr(align(4))]
union Vector {
    func: *const fn(),
    ext: *const usize,
    null: usize
}

unsafe impl Sync for Vector {}

macro_rules! vector {
    (fn $e:expr) => {
        Vector { func: $e as _}
    };
    (extern $e:expr) => {
        Vector { ext: unsafe {&$e} as *const _}
    };
    (static $e:expr) => {
        Vector { null: $e }
    };
    (null) => {
        vector!(static 0)
    }
}

macro_rules! const_vec {
    ($n:expr, $def: expr, $($b:expr), *) => {
        const {
            let mut arr = [$def; $n];
            let mut p = 0;

            $(
                #[allow(unused_assignments)]
                {
                    arr[p] = $b;
                    p += 1;
                }
            )*;

            arr
        }
    }
}

// Cortex-M4 Vector Table
#[link_section = ".vector_table"]
#[no_mangle]
#[used]
static VECTOR : [Vector; 250] = const_vec!(250, vector!(null),
    // Initial Stack Pointer
    vector!(extern LD_STACK_PTR),
    // Reset vector
    vector!(fn main)
);

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
pub fn main()  {}

#[no_mangle]
pub fn busy() -> ! {
    loop {}
}

