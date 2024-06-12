// src/main.rs

mod q1_palindrome_or_not;
mod q2_find_first_occurrence;
mod q3_shortest_word;
mod q4_is_prime;
mod q5_median_of_array;
mod q6_longest_common_prefix;
mod q7_kth_smallest_integer;
mod q8_max_depth_binarytree;
mod q9_reverse_string;
mod q10_prime_or_not;
mod q11_merge2arrays;
mod q12_max_subarray_sum;
fn main() {
    let choice = 12; // Change this value to 1 to run palindrome_or_not

    match choice {
        1 => q1_palindrome_or_not::run(),
        2 => q2_find_first_occurrence::run(),
        3 => q3_shortest_word::run(),
        4 => q4_is_prime::run(),
        5 => q5_median_of_array::run(),
        6 => q6_longest_common_prefix::run(),
        7 => q7_kth_smallest_integer::run(),
        8 => q8_max_depth_binarytree::run(),
        9 => q9_reverse_string::run(),
        10 => q10_prime_or_not::run(),
        11 => q11_merge2arrays::run(),
        12 => q12_max_subarray_sum::run(),
        _ => println!("Invalid choice"),
    }
}
