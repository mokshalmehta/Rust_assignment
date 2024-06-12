// Question 9: Reverse a string in Rust

pub fn run() {
    println!("Enter a string:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    let reversed_string = reverse_string(input.trim());
    println!("Reversed string: {}", reversed_string);
}

pub fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}
