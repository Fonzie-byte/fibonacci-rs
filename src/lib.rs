/// Returns the nth number from the Fibonacci sequence.
///
/// # Examples
/// ```
/// use fibonacci::recursive_fibonacci;
///
/// assert_eq!(recursive_fibonacci(0), 0);
/// assert_eq!(recursive_fibonacci(1), 1);
/// assert_eq!(recursive_fibonacci(2), 1);
/// assert_eq!(recursive_fibonacci(3), 2);
/// assert_eq!(recursive_fibonacci(4), 3);
/// assert_eq!(recursive_fibonacci(5), 5);
/// assert_eq!(recursive_fibonacci(12), 144);
/// ```
pub fn recursive_fibonacci(n: u32) -> usize {
    match n {
        0 => 0,
        1 => 1,
        _ => recursive_fibonacci(n - 1) + recursive_fibonacci(n - 2),
    }
}
