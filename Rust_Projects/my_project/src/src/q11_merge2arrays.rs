// Question 11: Merge two sorted arrays in Rust


use std::io;

pub fn run() {
    println!("Enter the elements of the first sorted array separated by spaces:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read line");

    println!("Enter the elements of the second sorted array separated by spaces:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read line");

    let arr1: Vec<i32> = input1.trim().split_whitespace()
        .map(|s| s.parse().expect("Invalid input. Please enter valid integers"))
        .collect();

    let arr2: Vec<i32> = input2.trim().split_whitespace()
        .map(|s| s.parse().expect("Invalid input. Please enter valid integers"))
        .collect();

    let merged_array = merge_sorted_arrays(&arr1, &arr2);
    
    println!("Merged Array: {:?}", merged_array);
}

pub fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut merged_array = Vec::new();
    let (mut i, mut j) = (0, 0);

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] < arr2[j] {
            merged_array.push(arr1[i]);
            i += 1;
        } else {
            merged_array.push(arr2[j]);
            j += 1;
        }
    }

    merged_array.extend_from_slice(&arr1[i..]);
    merged_array.extend_from_slice(&arr2[j..]);

    merged_array
}
