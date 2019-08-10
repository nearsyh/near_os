# Near OS

This is a simple OS written in Rust. It follows [Philipp Oppermann's awesome blog](https://os.phil-opp.com).

# Build
```bash
cargo build --target thumbv7em-none-eabihf
```

The ```target``` part is for compiling for bare metal systems. Note that it is possible to compile by adding some arguments for the linter, but it is not recommended because it may still uses some features from the C runtime.