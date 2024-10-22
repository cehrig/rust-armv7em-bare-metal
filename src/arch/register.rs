use core::marker::PhantomData;
use core::ops::Deref;
use core::ptr::{addr_of, read_volatile};

pub(crate) struct Register<T> {
    inner: PhantomData<T>,
}

unsafe impl<T> Sync for Register<T> {}

pub trait ReadPointer<T>
where
    T: RegisterOp,
{
    fn content(&self) -> T::RegisterWidth;
}

impl<T> ReadPointer<T> for *const Register<T>
where
    T: RegisterOp,
{
    fn content(&self) -> T::RegisterWidth {
        unsafe { (**self).ct() }
    }
}

impl<T> Register<T>
where
    T: RegisterOp,
{
    pub(crate) fn addr_self(&self) -> *const T::RegisterWidth {
        addr_of!(self.inner) as _
    }

    pub(crate) fn ct(&self) -> T::RegisterWidth {
        unsafe { read_volatile(self.addr_self()) }
    }
}

pub(crate) struct Wrapper<T>(pub *const Register<T>);

impl<T> Deref for Wrapper<T> {
    type Target = Register<T>;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}

pub(crate) struct GPIO;

pub(crate) trait RegisterOp {
    type RegisterWidth;
}

impl RegisterOp for GPIO {
    type RegisterWidth = usize;
}

impl GPIO {
    pub(crate) const BLA: usize = 0xFF;
}
