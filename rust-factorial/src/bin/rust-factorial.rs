
/*
cargo run rust-factorial
*/

extern crate rust_factorial;

use std::env;
use std::io::Write;
use std::str::FromStr;


fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}, {:?}", args, args.len());
    if args.len() != 2 {
        writeln!(std::io::stderr(),
                 "Usage: rust-factorial NUMBER")
            .unwrap();
        writeln!(std::io::stderr(),
                 "Example: {} 4",
                 args[0])
            .unwrap();
        std::process::exit(1);
    }

    let n: u64 = match u64::from_str(&args[1]) {
        Ok(nr) => nr,
        _ => {
            writeln!(std::io::stderr(), "Usage: rust-factorial NUMBER") .unwrap();
            writeln!(std::io::stderr(), "The number you've passed: '{}' could not be parsed!", &args[1]) .unwrap();
            std::process::exit(1);
        }
    };
    let result = rust_factorial::factorial(n);
    println!("{:?}", result);
}