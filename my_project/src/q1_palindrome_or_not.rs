// Question 1: Implement a function that checks whether a given string is a palindrome or not.

use std::io;

pub fn run() {
    println!("Enter a string:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();

    if is_palindrome(input) {
        println!("'{}' is a palindrome", input);
    } else {
        println!("'{}' is not a palindrome", input);
    }
}

fn is_palindrome(s: &str) -> bool {
    let clean_s = s.chars().filter(|c| c.is_alphanumeric()).collect::<String>();
    let reversed_clean_s = clean_s.chars().rev().collect::<String>();
    clean_s.eq_ignore_ascii_case(&reversed_clean_s)
}

