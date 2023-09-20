# Eww Utils

A set of utilites for my taskbar.

# Build and Run

This project uses cargo!

~~**NOTE:** In order to compile this code you have to pass the `RUSTFLAGS="-Cprefer-dynamic"`. See [this GitHub Issue](https://github.com/rust-lang/rust/issues/34909).~~

**NOTE:** Packaging a dynamic library in rust is a pain. Additionally memory usage actually increased, so...

```sh
cargo build --release
```
