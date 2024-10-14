#[derive(Copy, Clone)]
pub enum Vector {
    Func(&'static str, *const fn()),
    Ext(&'static str, *const usize),
    Null(&'static str, usize)
}

pub struct VectorTable<T, const N: usize> {
    inner: [T; N]
}

unsafe impl Sync for Vector {}

unsafe impl<T, const N: usize> Sync for VectorTable<T, N> {}

impl<const N: usize> VectorTable<Vector, N> {
    pub const fn new(default: Vector) -> Self {
        VectorTable {
            inner: [default; N]
        }
    }

    pub const fn set(mut self, n: usize, v: Vector) -> Self {
        self.inner[n] = v;
        self
    }

    pub fn get(&self, n: usize) -> Vector {
        self.inner[n]
    }

    pub const fn raw(&self) -> VectorTable<*const (), N> {
        let mut raw = [0 as *const (); N];
        let mut i = N;

        while i != 0 {
            raw[i-1] = match self.inner[i-1] {
                Vector::Func(_, v) => v as _,
                Vector::Ext(_, v) => v as _,
                Vector::Null(_, v) => v as *const _,
            };
            i -= 1;
        }

        VectorTable {inner: raw}
    }
}

#[macro_export]
macro_rules! vector {
    ($n:expr, fn $e:expr) => {
        Vector::Func($n, $e as _)
    };
    ($n:expr, extern $e:expr) => {
        Vector::Ext($n, unsafe {&$e} as *const _)
    };
    ($n:expr, static $e:expr) => {
        Vector::Null($n, $e)
    };
    (null) => {
        vector!("undef", static 0)
    }
}

#[macro_export]
macro_rules! const_vec {
    ($n:expr, $def: expr, $($b:expr), *) => {
        const {
            let mut t = VectorTable::<Vector, $n>::new($def);
            let mut p = 0;

            $(
                #[allow(unused_assignments)]
                {
                    t = t.set(p, $b);
                    p += 1;
                }
            )*;

            t
        }
    }
}
