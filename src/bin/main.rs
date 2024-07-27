use core::str;
use std::{io, io::Write, thread};
use std::time::Duration;
use lipsum::lipsum;

/// Prints the given string like a keyboard, character by character, with
/// random delays between each character.
///
/// # Arguments
///
/// * `s` - The string to print.
pub fn print_like_keyboard(s: &str) {
    for c in s.chars() {
        print!("{}", c);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(rand::random::<u64>() % 200));
    }
    println!();
}

fn main() {
    print_like_keyboard(&lipsum(20));
}
