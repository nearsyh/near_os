# Near OS

This is a simple OS written in Rust. It follows [Philipp Oppermann's awesome blog](https://os.phil-opp.com).

# Build
```bash
# This is required because we want to build for target x86_64-near_os.
cargo install cargo-xbuild

# Install bootimage so that we can link bootloader to build a bootable image.
cargo install bootimage --version "^0.7.3"

# Add llvm-tools-preview
rustup component add llvm-tools-preview

# With .cargo/config, cargo xbuild is enough.
# cargo xbuild --target x86_64-near_os.json
# cargo xbuild

# Compile all and create a bootable disk image
cargo bootimage
```

The ```target``` part is for compiling for bare metal systems. Note that it is possible to compile by adding some arguments for the linter, but it is not recommended because it may still uses some features from the C runtime.

# Architecture
```json
{
    // Target triple: architecture - vendor - operating system
    "llvm-target": "x86_64-unknown-none",

    // Specify type's size
    "data-layout": "e-m:e-i64:64-f80:128-n8:16:32:64-S128",

    "arch": "x86_64",
    "target-endian": "little",

    // Conditional compilation?
    "target-pointer-width": "64",

    "target-c-int-width": "32",
    "os": "none",
    "executables": true,

    // Use cross platform LLD linker
    "linker-flavor": "ld.lld",
    "linker": "rust-lld",

    // Panic strategy. Use abort so that we don't need stack unwinding.
    "panic-strategy": "abort",

    // Disable redzone optimization
    "disable-redzone": true,

    // mmx and sse are used for SIMD. Using SIMD in kernel impacts performace
    // because kernel needs to save SIMD state in registers during context
    // switch and SIMD state is large.
    // After disabling SIMD, we need to add soft-float feature.
    "features": "-mmx,-sse,+soft-float"
}
```