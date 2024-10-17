This is a toy project containing a Bare Metal Rust program that will eventually run on a STM32

# Build

```
$ cargo rustc -Zbuild-std=core --bin main --release
       Fresh core v0.0.0 (/home/cehrig/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core)
       Fresh rustc-std-workspace-core v1.99.0 (/home/cehrig/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/rustc-std-workspace-core)
       Fresh compiler_builtins v0.1.133
       Fresh asm v0.1.0 (/home/cehrig/Development/asm)
    Finished `release` profile [optimized] target(s) in 0.02s
```