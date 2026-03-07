use std::io::{self, Write};

/// Reads input from the user with a given prompt.
///
/// # Panics
///
/// Panics if:
/// - Flushing stdout fails
/// - Reading from stdin fails  
/// - The input cannot be parsed into type `T`
#[must_use]
pub fn read_input<T: std::str::FromStr>(prompt: &str) -> T
where
    T::Err: std::fmt::Debug,
{
    print!("{prompt}");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}
