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

/// Returns the nth number from the Fibonacci sequence, should be much faster than the recursive version.
///
/// # Examples
/// ```
/// use fibonacci::iterative_fibonacci;
///
/// assert_eq!(iterative_fibonacci(0), 0);
/// assert_eq!(iterative_fibonacci(1), 1);
/// assert_eq!(iterative_fibonacci(2), 1);
/// assert_eq!(iterative_fibonacci(3), 2);
/// assert_eq!(iterative_fibonacci(4), 3);
/// assert_eq!(iterative_fibonacci(5), 5);
/// assert_eq!(iterative_fibonacci(12), 144);
/// assert_eq!(iterative_fibonacci(90), 2_880_067_194_370_816_120);
/// ```
pub fn iterative_fibonacci(n: u32) -> usize {
    if n < 2 {
        return n as usize;
    }

    let mut previous = 0;
    let mut current = 1;

    for _ in 1..n {
        let next = previous + current;
        previous = current;
        current = next;
    }

    current
}
