#[cfg_attr(st32f4xx, path = "st32f4xx.rs")]
#[cfg_attr(st32f7xx, path = "st32f7xx.rs")]
mod _target;

pub use _target::*;

extern "C" {
    static LD_STACK_PTR: usize;
}
