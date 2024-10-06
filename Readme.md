This is a toy project containing a Bare Metal Rust program that can be executed in Qemu.

# Build
```
$ cargo build -Zbuild-std=core --bin main --release --target ./aarch64-unknown-none.json --verbose
       Fresh core v0.0.0 (/home/cehrig/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core)
       Fresh rustc-std-workspace-core v1.99.0 (/home/cehrig/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/rustc-std-workspace-core)
       Fresh compiler_builtins v0.1.130
       Fresh asm v0.1.0 (/home/cehrig/Development/asm)
    Finished `release` profile [optimized] target(s) in 0.01s
```

# Run
```
$ qemu-system-aarch64 \
    -machine virt \
    -m 512M \
    -smp 1 \
    -display none \
    -nographic \
    -cpu cortex-a53 \
    -kernel target/aarch64-unknown-none/release/main

Welcome
```

Alternatively

```
$ aarch64-linux-gnu-objcopy \
    -O binary \
    target/aarch64-unknown-none/release/main \
    target/aarch64-unknown-none/release/bios
    
$ qemu-system-aarch64 \
    -machine virt \
    -m 512M \
    -smp 1 \
    -display none \
    -nographic \
    -cpu cortex-a53 \
    -bios target/aarch64-unknown-none/release/bios
```