
# Hello World

[rustup](https://www.rustup.rs/)

    > cargo new --bin hello-world --name hello-world
    > cd hello-world
    > cargo run
    > cargo build --release
    > ./target/release/hello-world

# Simple library

    > cargo new --lib rust-factorial --name rust_factorial
    > idea.sh rust-factorial
    > cd rust-factorial

Have a look at [Rust IDEs](https://forge.rust-lang.org/ides.html) for details about which IDEs exist for Rust.

    > cargo run --bin rust-factorial 5
    > cargo build --lib

## Foreign-Function Interface (FFI): Calling Rust from C and calling C from Rust

### C++ version

    > cmake --build cmake-build-debug --target cc_factorial -- -j 4
    > cmake-build-debug/cc_factorial 4

    > cd cmake-build-debug
    > cmake ../
    > make
