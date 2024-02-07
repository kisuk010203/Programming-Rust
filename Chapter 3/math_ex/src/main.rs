fn fibonacci(n: u32) -> u32 {
    if n <= 2 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
fn collatz_length(mut n: i32) -> u32 {
    if n == 1 {
        0
    } else if n % 2 == 0 {
        1 + collatz_length(n / 2)
    } else {
        1 + collatz_length(3 * n + 1)
    }
}
#[test]
fn test_fibonacci_and_collatz() {
    assert_eq!(fibonacci(28), 317811);
    assert_eq!(collatz_length(27), 111);
}
fn main() {
    println!("Hello, world!");
}
