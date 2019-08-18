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

# Run
```bash
cargo run
```

# Test
```bash
cargo xtest
```

Current we have two kinds of tests: unit tests and integration tests. Unit tests are test cases written in each rs files under src directory. Integration tests are those in tests directory.

**Note**: Integration tests are run as separate executables.

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

# CPU Exceptions
There about 20 different CPU exceptions. We need to have exception handlers for them. The most important exceptions are:

* Page Fault: Caused by illegal memory access.
* Invalid Opcode
* General Protection Fault: Caused by access violations, e.g. run privileged instructions in user level.
* Double Fault: Caused by exception happens in exception handler.
* Triple Fault: Caused by exception happens in double fault handler. Most processors don't handle it but just reboot.

## Interrupt Descriptor Table (IDT)
Table for looking up handler functions by exceptions. Each entry's size is 16 bytes. The entry format is like this:

|   Type    |          Name            |                      Description                      |
|-----------|--------------------------|-------------------------------------------------------|
|    u16    | Function Pointer [0:15]  | Lower bits of the pointer to the handler function     |
|    u16    | GDT Selector             | Selector of a code segment in global descriptor table |
|    u16    | Options                  | (See below)                                           |
|    u16    | Function Pointer [16:31] | Middle bits of the pointer to the handler function    |
|    u32    | Function Pointer [32:63] | Remaining bits of the pointer to the handler function |
|    u32    | Reserved                 |                                                       |

The option field's format is:

| Bits  |         Name                   |                      Description                      |
|-------|--------------------------------|-------------------------------------------------------|
| 0-2   | Interrupt Stack Table Index    | 0: Don't switch stacks, 1-7: Switch to n-th stack in the Interrupt Stack Table when this handler is called |
| 3-7   | Reserved                       |                                                       |
| 8     | 0:Interrupt Gate, 1: Trap Gate | Interrupts are disabled is this bit is 0              |
| 9-11  | Must be one                    |                                                       |
| 12    | Must be zero                   |                                                       |
| 13-14 | Descriptor Privilege Level     | Minimal privilege level required to call this handler |
| 15    | Present                        |                                                       |