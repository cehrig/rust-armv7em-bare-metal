#![no_std]

pub mod arch;
pub mod ptr;

// Dummy placeholders
#[no_mangle]
pub fn main() -> ! {
    loop {}
}

#[no_mangle]
pub fn irq_default() -> ! {
    loop {}
}
