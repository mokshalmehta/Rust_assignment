// Question 6: Implement a function that finds the longest common prefix of a given set of strings.


use std::io;

pub fn run() {
    println!("Enter a set of strings separated by spaces:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let strings: Vec<&str> = input.trim().split_whitespace().collect();

    match find_longest_common_prefix(&strings) {
        Some(prefix) => println!("The longest common prefix is: '{}'", prefix),
        None => println!("No common prefix found"),
    }
}

pub fn find_longest_common_prefix(strings: &[&str]) -> Option<String> {
    if strings.is_empty() {
        return None;
    }

    let mut prefix = String::new();
    for (i, c) in strings[0].chars().enumerate() {
        if strings[1..].iter().all(|s| s.chars().nth(i) == Some(c)) {
            prefix.push(c);
        } else {
            break;
        }
    }

    if prefix.is_empty() {
        None
    } else {
        Some(prefix)
    }
}
