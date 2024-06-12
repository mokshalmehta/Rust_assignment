// Question 2: Given a sorted array of integers, implement a function that returns the index of the first occurrence of a given number.

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

    println!("Enter the target number:");
    let mut target_input = String::new();
    io::stdin().read_line(&mut target_input).expect("Failed to read line");
    let target: i32 = target_input.trim().parse().expect("Invalid input. Please enter a valid integer");

    match find_first_occurrence(&sorted_array, target) {
        Some(index) => println!("The first occurrence of {} is at index {}", target, index),
        None => println!("{} is not in the array", target),
    }
}

pub fn find_first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len() - 1;
    let mut result = None;

    while left <= right {
        let mid = left + (right - left) / 2;
        if arr[mid] == target {
            result = Some(mid);
            right = mid - 1; // Look on the left side for the first occurrence
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    result
}
