use fibonacci::iterative_fibonacci;
use std::env;

fn main() {
    let n = env::args()
        .nth(1)
        .expect("Please provide an argument.")
        .parse::<u32>()
        .expect("The first argument must be a positive integer.");
    let fibonacci = iterative_fibonacci(n);

    println!("The {}th Fibonacci number is {}", n, fibonacci);
}
