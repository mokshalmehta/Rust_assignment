// Question 7 : Implement a function that returns the kth smallest element in a given array.


use std::io;

pub fn run() {
    println!("Enter integers separated by spaces:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let array: Vec<i32> = input.trim().split_whitespace()
        .map(|s| s.parse().expect("Invalid input. Please enter valid integers"))
        .collect();

    println!("Enter the value of k:");
    let mut k_input = String::new();
    io::stdin().read_line(&mut k_input).expect("Failed to read line");
    let k: usize = k_input.trim().parse().expect("Invalid input. Please enter a valid integer");

    match find_kth_smallest(&array, k) {
        Some(kth_smallest) => println!("The {}th smallest element is: {}", k, kth_smallest),
        None => println!("Invalid value of k"),
    }
}

pub fn find_kth_smallest(array: &[i32], k: usize) -> Option<i32> {
    if k == 0 || k > array.len() {
        return None;
    }

    let mut sorted_array = array.to_vec();
    sorted_array.sort();

    Some(sorted_array[k - 1])
}
