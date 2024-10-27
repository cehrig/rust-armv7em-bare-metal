use core::marker::PhantomData;
use core::ops::BitAnd;
use core::ops::{BitOrAssign, Shl};
use core::ptr::write_volatile;
use core::ptr::{addr_of, read_volatile};

pub(crate) struct BitArray<const N: usize>([u8; N]);

pub(crate) type Word = BitArray<4>;

pub(crate) type HalfWord = BitArray<2>;

pub(crate) struct Register<T> {
    inner: PhantomData<T>,
}

pub(crate) struct BitsWithOffset<const O: usize, const S: usize, const E: usize, Scalar>(
    PhantomData<Scalar>,
);

pub(crate) struct BitsIterator {
    offset: usize,
    start: usize,
    end: usize,
}

pub(crate) enum Assert<const CHECK: bool> {}

pub(crate) trait RegisterOps<T> {
    fn read(&self) -> T;

    fn write(&self, _: T);

    fn get<O>(&self, _: O) -> O::Scalar
    where
        O: BitsOffset,
        O::Scalar: BitScalar + BitOrAssign<O::Scalar> + Shl<usize, Output = O::Scalar>,
        Assert<{ <O as BitsOffset>::MAX_BYTE_OFFSET < size_of::<T>() }>: IsTrue;

    fn set<O>(&self, _: O)
    where
        O: BitsOffset,
        O::Scalar: BitScalar
            + BitAnd<O::Scalar, Output = O::Scalar>
            + Shl<usize, Output = O::Scalar>
            + PartialOrd
            + Copy,
        Assert<{ <O as BitsOffset>::MAX_BYTE_OFFSET < size_of::<T>() }>: IsTrue;

    fn set_from<O>(&self, _: O, _: O::Scalar)
    where
        O: BitsOffset,
        O::Scalar: BitScalar
            + BitAnd<O::Scalar, Output = O::Scalar>
            + Shl<usize, Output = O::Scalar>
            + PartialOrd
            + Copy,
        Assert<{ <O as BitsOffset>::MAX_BYTE_OFFSET < size_of::<T>() }>: IsTrue;

    fn clear<O>(&self, _: O)
    where
        O: BitsOffset,
        Assert<{ <O as BitsOffset>::MAX_BYTE_OFFSET < size_of::<T>() }>: IsTrue;

    fn all_set<O>(&self, _: impl IntoIterator<Item = O>) -> bool
    where
        O: BitsOffset,
        Assert<{ <O as BitsOffset>::MAX_BYTE_OFFSET < size_of::<T>() }>: IsTrue;
}

pub(crate) trait RegisterBase {
    type Base;

    fn to_base(self) -> Self::Base;

    fn from_base(_: Self::Base) -> Self;
}

pub(crate) trait Bits {
    fn set<O, S>(&mut self, _: O, _: S)
    where
        O: BitsOffset,
        S: BitScalar + BitAnd<S, Output = S> + Shl<usize, Output = S> + PartialOrd + Copy;

    fn get<O, S>(&self, _: O) -> S
    where
        O: BitsOffset,
        S: BitScalar + BitOrAssign<S> + Shl<usize, Output = S>;

    fn all_set<O>(&self, _: O) -> bool
    where
        O: BitsOffset;

    fn clear<O>(&mut self, _: O)
    where
        O: BitsOffset;
}

pub(crate) trait BitScalar {
    fn bit() -> Self;

    fn empty() -> Self;

    fn all() -> Self;
}

pub(crate) trait BitsOffset {
    type Scalar;

    const BYTE_OFFSET: usize;

    const START: usize;

    const END: usize;

    const MAX_BYTE_OFFSET: usize = {
        let mut offset = Self::BYTE_OFFSET;
        let mut end = Self::END;

        while end >= 8 {
            offset += 1;
            end -= 8
        }

        offset
    };

    fn iter(&self) -> BitsIterator;
}

pub(crate) trait IsTrue {}

impl<const N: usize> RegisterBase for BitArray<N> {
    type Base = BitArray<N>;

    fn to_base(self) -> Self::Base {
        self
    }

    fn from_base(base: Self::Base) -> Self {
        base
    }
}

unsafe impl<T> Sync for Register<T> {}

impl<T> RegisterOps<T> for *const Register<T>
where
    T: RegisterBase,
    T::Base: Bits,
{
    fn read(&self) -> T {
        unsafe { (**self).read() }
    }

    fn write(&self, src: T) {
        unsafe { (**self).write(src) }
    }

    fn get<O>(&self, bits: O) -> O::Scalar
    where
        O: BitsOffset,
        O::Scalar: BitScalar + BitOrAssign<O::Scalar> + Shl<usize, Output = O::Scalar>,
        Assert<{ <O as BitsOffset>::MAX_BYTE_OFFSET < size_of::<T>() }>: IsTrue,
    {
        let v = self.read().to_base();

        v.get(bits)
    }

    fn set<O>(&self, bits: O)
    where
        O: BitsOffset,
        O::Scalar: BitScalar
            + BitAnd<O::Scalar, Output = O::Scalar>
            + Shl<usize, Output = O::Scalar>
            + PartialOrd
            + Copy,
        Assert<{ <O as BitsOffset>::MAX_BYTE_OFFSET < size_of::<T>() }>: IsTrue,
    {
        self.set_from(bits, <O::Scalar as BitScalar>::all())
    }

    fn set_from<O>(&self, bits: O, value: O::Scalar)
    where
        O: BitsOffset,
        O::Scalar: BitScalar
            + BitAnd<O::Scalar, Output = O::Scalar>
            + Shl<usize, Output = O::Scalar>
            + PartialOrd
            + Copy,
        Assert<{ <O as BitsOffset>::MAX_BYTE_OFFSET < size_of::<T>() }>: IsTrue,
    {
        let mut v = self.read().to_base();
        v.set(bits, value);

        self.write(RegisterBase::from_base(v))
    }

    fn clear<O>(&self, bits: O)
    where
        O: BitsOffset,
        Assert<{ <O as BitsOffset>::MAX_BYTE_OFFSET < size_of::<T>() }>: IsTrue,
    {
        let mut v = self.read().to_base();
        v.clear(bits);

        self.write(RegisterBase::from_base(v))
    }

    fn all_set<O>(&self, bits: impl IntoIterator<Item = O>) -> bool
    where
        O: BitsOffset,
        Assert<{ <O as BitsOffset>::MAX_BYTE_OFFSET < size_of::<T>() }>: IsTrue,
    {
        let v = self.read().to_base();

        for i in bits {
            if !v.all_set(i) {
                return false;
            }
        }

        true
    }
}

impl<T> Register<T> {
    fn addr_self(&self) -> *const T {
        addr_of!(self.inner) as _
    }

    pub(crate) fn read(&self) -> T {
        unsafe { read_volatile(self.addr_self()) }
    }

    pub(crate) fn write(&self, src: T) {
        unsafe { write_volatile(self.addr_self() as *mut _, src) }
    }
}

impl<const N: usize> Bits for BitArray<N> {
    fn set<O, S>(&mut self, bits: O, value: S)
    where
        O: BitsOffset,
        S: BitScalar + BitAnd<S, Output = S> + Shl<usize, Output = S> + PartialOrd + Copy,
    {
        let iter = bits.iter();

        for (source, (offset, bit)) in iter.enumerate() {
            if value & S::bit() << source > S::empty() {
                self.set_bit(offset, bit)
            } else {
                self.clear_bit(offset, bit)
            }
        }
    }

    fn get<O, S>(&self, bits: O) -> S
    where
        O: BitsOffset,
        S: BitScalar + BitOrAssign<S> + Shl<usize, Output = S>,
    {
        let mut result = S::empty();
        let iter = bits.iter();

        for (target, (offset, bit)) in iter.enumerate() {
            if self.is_set(offset, bit) {
                result |= S::bit() << target;
            }
        }

        result
    }

    fn all_set<O>(&self, bits: O) -> bool
    where
        O: BitsOffset,
    {
        let iter = bits.iter();

        for (offset, bit) in iter {
            if !self.is_set(offset, bit) {
                return false;
            }
        }

        true
    }

    fn clear<O>(&mut self, bits: O)
    where
        O: BitsOffset,
    {
        let iter = bits.iter();

        for (_, (offset, bit)) in iter.enumerate() {
            self.clear_bit(offset, bit);
        }
    }
}

impl<const N: usize> BitArray<N> {
    fn offset_ok(&self, offset: usize) -> bool {
        offset < N
    }

    fn is_set(&self, offset: usize, bit: usize) -> bool {
        if !self.offset_ok(offset) {
            return false;
        }

        self.0[offset] & (1 << bit) > 0
    }

    fn set_bit(&mut self, offset: usize, bit: usize) {
        if !self.offset_ok(offset) {
            return;
        }

        self.0[offset] |= 1 << bit
    }

    fn clear_bit(&mut self, offset: usize, bit: usize) {
        if !self.offset_ok(offset) {
            return;
        }

        self.0[offset] &= !(1 << bit)
    }
}

impl<const O: usize, const S: usize, const E: usize, Scalar> BitsWithOffset<O, S, E, Scalar>
where
    Scalar: BitScalar,
{
    const CHECK: () = { assert!(E - S <= size_of::<Scalar>() * 8) };

    pub(crate) const fn new() -> Self {
        let _ = Self::CHECK;

        BitsWithOffset(PhantomData)
    }
}

impl<const O: usize, const S: usize, const E: usize, Scalar> BitsOffset
    for BitsWithOffset<O, S, E, Scalar>
{
    type Scalar = Scalar;

    const BYTE_OFFSET: usize = O;
    const START: usize = S;
    const END: usize = E;

    fn iter(&self) -> BitsIterator {
        self.into_iter()
    }
}

impl<'a, const O: usize, const S: usize, const E: usize, Scalar> IntoIterator
    for &'a BitsWithOffset<O, S, E, Scalar>
{
    type Item = (usize, usize);
    type IntoIter = BitsIterator;

    fn into_iter(self) -> Self::IntoIter {
        BitsIterator::new(O, S, E)
    }
}

impl BitsIterator {
    const fn new(offset: usize, start: usize, end: usize) -> Self {
        let (offset, start, end) = Self::normalize(offset, start, end);

        BitsIterator { offset, start, end }
    }

    const fn normalize(
        mut offset: usize,
        mut start: usize,
        mut end: usize,
    ) -> (usize, usize, usize) {
        while start >= 8 {
            offset += 1;
            start -= 8;
            end -= 8;
        }

        (offset, start, end)
    }
}

impl Iterator for BitsIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            return None;
        }

        (self.offset, self.start, self.end) =
            BitsIterator::normalize(self.offset, self.start, self.end);

        self.start += 1;

        Some((self.offset, self.start - 1))
    }
}

impl<const O: usize, const S: usize, const E: usize, Scalar> IntoIterator
    for BitsWithOffset<O, S, E, Scalar>
{
    type Item = BitsWithOffset<O, S, E, Scalar>;
    type IntoIter = core::array::IntoIter<BitsWithOffset<O, S, E, Scalar>, 1>;

    fn into_iter(self) -> Self::IntoIter {
        [self].into_iter()
    }
}

impl IsTrue for Assert<true> {}

macro_rules! bitscalar_impl_for_num {
    ($($ty:ty),*) => {
        $(
            impl BitScalar for $ty {
                fn bit() -> Self {
                    1
                }

                fn empty() -> Self {
                    0
                }

                fn all() -> Self {
                    Self::MAX
                }
            }
        )*
    };
}

bitscalar_impl_for_num!(u8, u16, u32, u64, u128, usize);
