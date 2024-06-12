// Question 11: Find the maximum subarray sum in Rust

pub fn run() {
    println!("Enter the elements of the array separated by spaces:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    let array: Vec<i32> = input.trim().split_whitespace()
        .map(|s| s.parse().expect("Invalid input. Please enter valid integers"))
        .collect();

    let max_sum = max_subarray_sum(&array);
    println!("Maximum subarray sum: {}", max_sum);
}

pub fn max_subarray_sum(nums: &[i32]) -> i32 {
    let mut max_sum = nums[0];
    let mut current_sum = nums[0];

    for &num in nums.iter().skip(1) {
        current_sum = current_sum.max(0) + num;
        max_sum = max_sum.max(current_sum);
    }

    max_sum
}
