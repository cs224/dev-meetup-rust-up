
# DevMeetup: Quick intro to Rust.

Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.

* [rust-lang](https://www.rust-lang.org/en-US/)
* [wikipedia](https://en.wikipedia.org/wiki/Rust_%28programming_language%29)

# Installation of the Rust tool-chain

Just follow the instructions here: [rustup](https://www.rustup.rs/).

# Hello World

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

    > cd ffi-cc-from-rust
    > cargo build
    > ./target/debug/ffi-cc-from-rust 4

# Type system

* [structs](https://rustbyexample.com/custom_types/structs.html)
* [traits](https://rustbyexample.com/trait.html)
* [trait-objects](https://doc.rust-lang.org/book/first-edition/trait-objects.html)
* [generics](https://doc.rust-lang.org/book/first-edition/generics.html)


    > cd rust-shapes
    > cargo build
    > cargo test

# Application with multi-threading

* [ProgrammingRust/mandelbrot](https://github.com/ProgrammingRust/mandelbrot)


    > git clone https://github.com/ProgrammingRust/mandelbrot.git mandelbrot-single-threaded
    > cd mandelbrot-single-threaded
    > git checkout single-threaded
    > rm Cargo.lock
    > cargo build --release
    > time ./target/release/mandelbrot mandel.png 10000x7500 -1.20,0.35 -1,0.20

Here is the output:
```
$ time ./target/release/mandelbrot mandel.png 10000x7500 -1.20,0.35 -1,0.20
real    0m23.175s
user    0m23.130s
sys     0m0.045s
```

And now the same with multi-threading:
    > git clone https://github.com/ProgrammingRust/mandelbrot.git mandelbrot-rayon
    > cd mandelbrot-rayon
    > git checkout rayon
    > rm Cargo.lock
    > cargo build --release
    > time ./target/release/mandelbrot mandel.png 10000x7500 -1.20,0.35 -1,0.20


Here is the output:
```
$ time ./target/release/mandelbrot mandel.png 10000x7500 -1.20,0.35 -1,0.20
real    0m2.805s
user    0m29.904s
sys     0m0.164s
```
So on my machine this corresponds to a speed-up of roughly 8.25 (with 6 real cores). A big junk of the work is actually writing the image to disk, which is always single-threaded.

The only difference between the two programs is this code block Here

```rust
render(&mut pixels[..], bounds, upper_left, lower_right);
```

```rust
// Scope of slicing up `pixels` into horizontal bands.
{
    let bands: Vec<(usize, &mut [u8])> = pixels
        .chunks_mut(bounds.0)
        .enumerate()
        .collect();

    bands.into_par_iter()
        .weight_max()
        .for_each(|(i, band)| {
            let top = i;
            let band_bounds = (bounds.0, 1);
            let band_upper_left = pixel_to_point(bounds, (0, top),
                                                 upper_left, lower_right);
            let band_lower_right = pixel_to_point(bounds, (bounds.0, top + 1),
                                                  upper_left, lower_right);
            render(band, band_bounds, band_upper_left, band_lower_right);
        });
}
```

# Exception/error handling in Rust

* [error-handling](https://doc.rust-lang.org/stable/book/first-edition/error-handling.html)
  * [trait-based-exception-handling](https://github.com/rust-lang/rfcs/blob/master/text/0243-trait-based-exception-handling.md)
    * [Rust: The `?` operator](https://m4rw3r.github.io/rust-questionmark-operator) and do-notation
  * [unrecoverable-errors-with-panic](https://doc.rust-lang.org/stable/book/second-edition/ch09-01-unrecoverable-errors-with-panic.html)
    * [suppress-panic-output-in-rust-when-using-panic::catch-unwind](https://stackoverflow.com/questions/35559267/suppress-panic-output-in-rust-when-using-paniccatch-unwind)

# Disappointments

* No tail-call optimization: [proper-tail-calls](https://github.com/DemiMarie/rfcs/blob/become/0000-proper-tail-calls.md)
* No goroutines/green-threads/fibers: [experimental-coroutines](https://github.com/rust-lang/rfcs/blob/master/text/2033-experimental-coroutines.md)
  * [Green Threads in Rust](http://cs242.stanford.edu/assets/projects/2017/kedero.pdf)
