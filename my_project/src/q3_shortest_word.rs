//Question 3: Given a string of words, implement a function that returns the shortest word in the string.


use std::io;

pub fn run() {
    println!("Enter a sentence:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let sentence = input.trim();

    match shortest_word(sentence) {
        Some(shortest) => println!("The shortest word is: '{}'", shortest),
        None => println!("No words found"),
    }
}

pub fn shortest_word(sentence: &str) -> Option<&str> {
    sentence.split_whitespace().min_by_key(|word| word.len())
}
