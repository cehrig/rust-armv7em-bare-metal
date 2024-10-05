use core::ptr;

#[inline(never)]
#[no_mangle]
pub fn write_slice(addr: *mut u8, slice: &[u8]) {
    for byte in slice {
        unsafe {
            ptr::write_volatile(addr, *byte);
        }
    }
}
