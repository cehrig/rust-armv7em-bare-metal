#[cfg_attr(all(target_arch = "arm", target_abi = "eabihf"), path = "st32f4xx.rs")]
#[cfg_attr(all(target_arch = "arm", target_abi = "eabi"), path = "st32f7xx.rs")]
mod _target;

pub use _target::*;
