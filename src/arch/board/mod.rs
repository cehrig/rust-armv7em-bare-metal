#[cfg(feature = "st32f4xx")]
pub mod st32f4xx;

#[cfg(feature = "st32f4xx")]
pub use st32f4xx::*;

#[cfg(feature = "st32f7xx")]
pub mod st32f7xx;

#[cfg(feature = "st32f7xx")]
pub use st32f7xx::*;

extern "C" {
    pub static LD_STACK_PTR: usize;
}
