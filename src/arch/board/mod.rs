#[cfg(feature = "st32f4xx")]
mod st32f4xx;

#[cfg(feature = "st32f4xx")]
pub use st32f4xx::*;

#[cfg(feature = "st32f7xx")]
mod st32f7xx;

#[cfg(feature = "st32f7xx")]
pub use st32f7xx::*;
