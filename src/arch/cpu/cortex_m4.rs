// Configurable Fault Status Register (MMFSR, BFSR, UFSR)
pub const CFSR_ADDR: *const usize = 0xE000_ED28 as *const usize;

// Application Interrupt and Reset Control Register
pub const AIRCR_ADDR: *const usize = 0xE000_ED0C as *const usize;
