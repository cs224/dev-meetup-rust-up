

/*

https://stackoverflow.com/questions/40833078/how-do-i-specify-the-linker-path-in-rust

rustup show will tell you your target name that you will have to use in .cargo/config
> rustup show

https://doc.rust-lang.org/cargo/reference/build-scripts.html
https://stackoverflow.com/questions/40602708/linking-rust-application-with-a-dynamic-library-not-in-the-runtime-linker-search

https://doc.rust-lang.org/cargo/reference/environment-variables.html
https://doc.rust-lang.org/cargo/reference/config.html#environment-variables
RUSTFLAGS="-C link-args=-Wl,-rpath,../cc-factorial/cmake-build-debug" cargo build --verbose

*/


use std::env;
use std::io::Write;
use std::str::FromStr;

#[link(name = "cc_factorial")]
extern {
    fn factorial(n:u64) -> u64;
}


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
    let result = unsafe { factorial(n) };
    println!("{:?}", result);
}