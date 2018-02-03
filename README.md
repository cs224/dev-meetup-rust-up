
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

# Foreign-Function Interface (FFI): Calling Rust from C and calling C from Rust

## Raw C++ version

    > cd cc-factorial
    > mkdir cmake-build-debug
    > cd cmake-build-debug
    > cmake -DCMAKE_BUILD_TYPE=Debug -G "CodeBlocks - Unix Makefiles" .
    > cd ..

    > cmake --build cmake-build-debug --target cc_factorial -- -j 4
    > ./cmake-build-debug/cc_factorial 4

    > cd cmake-build-debug
    > cmake ../
    > make

## C++ calling Rust

    > cd ffi-rust-from-cc
    > mkdir cmake-build-debug
    > cd cmake-build-debug
    > cmake -DCMAKE_BUILD_TYPE=Debug -G "CodeBlocks - Unix Makefiles" .
    > make

## Rust calling C++



# Application with multi-threading

* [ProgrammingRust/mandelbrot](https://github.com/ProgrammingRust/mandelbrot)

    > git clone https://github.com/ProgrammingRust/mandelbrot.git mandelbrot-single-threaded
    > cd mandelbrot-single-threaded
    > git checkout single-threaded
    > rm Cargo.lock
    > cargo build --release
    > time ./target/release/mandelbrot mandel.png 10000x7500 -1.20,0.35 -1,0.20


    > git clone https://github.com/ProgrammingRust/mandelbrot.git mandelbrot-rayon
    > cd mandelbrot-rayon
    > git checkout rayon
    > rm Cargo.lock
    > cargo build --release
    > time ./target/release/mandelbrot mandel.png 10000x7500 -1.20,0.35 -1,0.20
