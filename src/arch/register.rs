use core::marker::PhantomData;
use core::ops::BitAnd;
use core::ops::Bound;
use core::ops::{BitOrAssign, RangeBounds, Shl};
use core::ptr::write_volatile;
use core::ptr::{addr_of, read_volatile};

pub(crate) struct BitArray<const N: usize>([u8; N]);

pub(crate) type Word = BitArray<4>;

pub(crate) struct Register<T> {
    inner: PhantomData<T>,
}

pub(crate) struct BitsWithOffset<R, S>(usize, R, PhantomData<S>);

pub(crate) struct BitsIterator {
    offset: usize,
    start: usize,
    end: usize,
}

pub(crate) trait RegisterOps<T> {
    fn read(&self) -> T;

    fn write(&self, _: T);

    fn get<O>(&self, _: O) -> O::Scalar
    where
        O: BitsOffset,
        O::Scalar: BitScalar + BitOrAssign<O::Scalar> + Shl<usize, Output = O::Scalar>;

    fn set<O>(&self, _: O)
    where
        O: BitsOffset,
        O::Scalar: BitScalar
            + BitAnd<O::Scalar, Output = O::Scalar>
            + Shl<usize, Output = O::Scalar>
            + PartialOrd
            + Copy;

    fn set_from<O>(&self, _: O, _: O::Scalar)
    where
        O: BitsOffset,
        O::Scalar: BitScalar
            + BitAnd<O::Scalar, Output = O::Scalar>
            + Shl<usize, Output = O::Scalar>
            + PartialOrd
            + Copy;

    fn clear<O>(&self, _: O)
    where
        O: BitsOffset,
        O::Scalar: BitScalar
            + BitAnd<O::Scalar, Output = O::Scalar>
            + Shl<usize, Output = O::Scalar>
            + PartialOrd
            + Copy;

    fn all_set<O>(&self, _: impl IntoIterator<Item = O>) -> bool
    where
        O: BitsOffset;
}

pub(crate) trait RegisterBase {
    type Base;

    fn to_base(self) -> Self::Base;

    fn from_base(_: Self::Base) -> Self;
}

pub(crate) trait BitRangeStartEnd {
    fn start(&self) -> usize;

    fn end(&self) -> usize;
}

pub(crate) trait Bits {
    fn is_set(&self, _: usize, _: usize) -> bool;

    fn set_bit(&mut self, _: usize, _: usize);

    fn clear_bit(&mut self, _: usize, _: usize);

    fn set<S>(&mut self, _: BitsIterator, _: S)
    where
        S: BitScalar + BitAnd<S, Output = S> + Shl<usize, Output = S> + PartialOrd + Copy;

    fn get<S>(&self, _: BitsIterator) -> S
    where
        S: BitScalar + BitOrAssign<S> + Shl<usize, Output = S>;

    fn clear(&mut self, _: BitsIterator);
}

pub(crate) trait BitScalar {
    fn bit() -> Self;

    fn empty() -> Self;

    fn all() -> Self;
}

pub(crate) trait BitsOffset {
    type Scalar;

    fn iter(&self) -> BitsIterator;
}

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
    {
        let v = self.read().to_base();

        v.get(bits.iter())
    }

    fn set<O>(&self, bits: O)
    where
        O: BitsOffset,
        O::Scalar: BitScalar
            + BitAnd<O::Scalar, Output = O::Scalar>
            + Shl<usize, Output = O::Scalar>
            + PartialOrd
            + Copy,
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
    {
        let mut v = self.read().to_base();
        v.set(bits.iter(), value);

        self.write(RegisterBase::from_base(v))
    }

    fn clear<O>(&self, bits: O)
    where
        O: BitsOffset,
        O::Scalar: BitScalar
            + BitAnd<O::Scalar, Output = O::Scalar>
            + Shl<usize, Output = O::Scalar>
            + PartialOrd
            + Copy,
    {
        let mut v = self.read().to_base();
        v.clear(bits.iter());

        self.write(RegisterBase::from_base(v))
    }

    fn all_set<O>(&self, bits: impl IntoIterator<Item = O>) -> bool
    where
        O: BitsOffset,
    {
        let v = self.read().to_base();

        for i in bits {
            for (offset, bit) in i.iter() {
                if !v.is_set(offset, bit) {
                    return false;
                }
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
    fn is_set(&self, offset: usize, bit: usize) -> bool {
        self.0[offset] & (1 << bit) > 0
    }

    fn set_bit(&mut self, offset: usize, bit: usize) {
        self.0[offset] |= 1 << bit
    }

    fn clear_bit(&mut self, offset: usize, bit: usize) {
        self.0[offset] &= !(1 << bit)
    }

    fn set<S>(&mut self, bits: BitsIterator, value: S)
    where
        S: BitScalar + BitAnd<S, Output = S> + Shl<usize, Output = S> + PartialOrd + Copy,
    {
        for (source, (offset, bit)) in bits.enumerate() {
            if value & S::bit() << source > S::empty() {
                self.set_bit(offset, bit)
            } else {
                self.clear_bit(offset, bit)
            }
        }
    }

    fn get<S>(&self, bits: BitsIterator) -> S
    where
        S: BitScalar + BitOrAssign<S> + Shl<usize, Output = S>,
    {
        let mut result = S::empty();

        for (target, (offset, bit)) in bits.enumerate() {
            if self.is_set(offset, bit) {
                result |= S::bit() << target;
            }
        }

        result
    }

    fn clear(&mut self, bits: BitsIterator) {
        for (_, (offset, bit)) in bits.enumerate() {
            self.clear_bit(offset, bit);
        }
    }
}

impl<R, S> BitsWithOffset<R, S>
where
    R: RangeBounds<usize>,
    S: BitScalar,
{
    // Todo: Width of range must fit into S
    pub(crate) const fn new(offset: usize, range: R) -> Self {
        BitsWithOffset(offset, range, PhantomData)
    }
}

impl<R, S> BitsOffset for BitsWithOffset<R, S>
where
    R: RangeBounds<usize>,
{
    type Scalar = S;

    fn iter(&self) -> BitsIterator {
        self.into_iter()
    }
}

impl<'a, R, S> IntoIterator for &'a BitsWithOffset<R, S>
where
    R: RangeBounds<usize>,
{
    type Item = (usize, usize);
    type IntoIter = BitsIterator;

    fn into_iter(self) -> Self::IntoIter {
        let offset = self.0;
        let start = self.1.start();
        let end = self.1.end();

        BitsIterator::new(offset, start, end)
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

impl<R> BitRangeStartEnd for R
where
    R: RangeBounds<usize>,
{
    fn start(&self) -> usize {
        match self.start_bound() {
            Bound::Included(s) => *s,
            Bound::Excluded(s) => s.saturating_add(1),
            _ => panic!("Bit range start must be bounded"),
        }
    }

    fn end(&self) -> usize {
        match self.end_bound() {
            Bound::Included(e) => *e,
            Bound::Excluded(e) => e.saturating_sub(1),
            _ => panic!("Bit range end must be bounded"),
        }
    }
}

impl Iterator for BitsIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.start > self.end {
            return None;
        }

        (self.offset, self.start, self.end) =
            BitsIterator::normalize(self.offset, self.start, self.end);

        self.start += 1;

        Some((self.offset, self.start - 1))
    }
}

impl<R, S> IntoIterator for BitsWithOffset<R, S> {
    type Item = BitsWithOffset<R, S>;
    type IntoIter = core::array::IntoIter<BitsWithOffset<R, S>, 1>;

    fn into_iter(self) -> Self::IntoIter {
        [self].into_iter()
    }
}

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
