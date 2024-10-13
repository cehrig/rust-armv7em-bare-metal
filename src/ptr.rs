use core::borrow::Borrow;
use core::ptr;


#[inline(never)]
pub fn write_slice<'a, F>(addr: *mut u8, slice: impl IntoIterator<Item = F>) where F: Borrow<u8> {
    for byte in slice {
        unsafe {
            ptr::write_volatile(addr, *byte.borrow());
        }
    }
}
