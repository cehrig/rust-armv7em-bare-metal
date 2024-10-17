use core::ptr;

#[inline(never)]
pub fn write_volatile<T>(addr: *mut T, slice: impl IntoIterator<Item = T>)
where
    T: Copy,
{
    for byte in slice {
        unsafe {
            ptr::write_volatile(addr, byte);
        }
    }
}

#[inline(never)]
pub fn read_volatile<T>(addr: *const T) -> T {
    unsafe { ptr::read_volatile(addr) }
}
