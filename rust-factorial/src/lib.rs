

#[no_mangle]
pub fn factorial(n:u64) -> u64 {
    let mut result = 1u64;
    for i in 1..(n+1) {
        result *= i;
        // println!("{}", result);
    }
    result
}

#[test]
fn test_factorial() {
    // assert_eq!(1, factorial(-1)); // would cause compile error and not a runtime error
    assert_eq!(1, factorial(0));
    assert_eq!(1, factorial(1));
    assert_eq!(2, factorial(2));
    assert_eq!(6, factorial(3));
    assert_eq!(24, factorial(4));
}

