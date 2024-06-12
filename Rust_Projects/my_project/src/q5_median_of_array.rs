// Question 5: Given a sorted array of integers, implement a function that returns the median of the array.

use std::io;

pub fn run() {
    println!("Enter a sorted array of integers separated by spaces:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let sorted_array: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input. Please enter valid integers"))
        .collect();

    match find_median(&sorted_array) {
        Some(median) => println!("The median of the array is: {}", median),
        None => println!("The array is empty"),
    }
}

pub fn find_median(sorted_array: &[i32]) -> Option<f64> {
    let len = sorted_array.len();
    if len == 0 {
        return None;
    }
    let mid = len / 2;
    if len % 2 == 0 {
        Some((sorted_array[mid - 1] + sorted_array[mid]) as f64 / 2.0)
    } else {
        Some(sorted_array[mid] as f64)
    }
}
