
# Rust Assignment

This project contains various algorithms implemented in Rust.

## Project Structure

The project directory structure is as follows:

```
Rust_project
|
Output      //Screenshot of every output
|
my_project
│ 
│
└───src
    │   main.rs
    │
    ├───q1_palindrome_or_not.rs
    ├───q2_find_first_occurrence.rs
    ├───q3_shortest_word.rs
    ├───q4_is_prime.rs
    ├───q5_median_of_array.rs
    ├───q6_longest_common_prefix.rs
    ├───q7_kth_smallest_integer.rs
    ├───q8_max_depth_binarytree.rs
    ├───q9_reverse_string.rs
    ├───q10_prime_or_not.rs
    ├───q11_merge2arrays.rs
    └───q12_max_subarray_sum.rs
```

## Instructions

### 1. Cloning the Repository

```

### 2. Running the Programs

#### a. Navigate to the Project Directory

```
cd my_project
```

#### b. Compile the Rust Files

Before running the programs, ensure that you have Rust installed on your system. If not, you can download and install it from [Rust's official website](https://www.rust-lang.org/tools/install).

Compile the Rust files using the following command:

```
cargo build --release
```

#### c. Running a Specific Program

After compiling the Rust files, you can run a specific program by changing the value of `choice` in the `main.rs` file.

For example, to run the program for finding the maximum subarray sum, set `choice = 12` in the `main.rs` file:

```rust
let choice = 12;
```

To run the program for finding the longest common prefix, set `choice = 6`:

```rust
let choice = 6;
```

After making the change, save the `main.rs` file and recompile and run the project using the following commands:

```
cargo build --release
cargo run
```

### 3. Available Programs

Below is a list of the algorithms implemented in this assignment along with their corresponding program names:

1. Palindrome Checker: `q1_palindrome_or_not`
2. First Occurrence of a Number in a Sorted Array: `q2_find_first_occurrence`
3. Shortest Word in a String: `q3_shortest_word`
4. Prime Number Checker: `q4_is_prime`
5. Median of an Array: `q5_median_of_array`
6. Longest Common Prefix: `q6_longest_common_prefix`
7. Kth Smallest Integer: `q7_kth_smallest_integer`
8. Maximum Depth of a Binary Tree: `q8_max_depth_binarytree`
9. Reverse a String: `q9_reverse_string`
10. Prime Number Checker (with user input): `q10_prime_or_not`
11. Merge Two Sorted Arrays: `q11_merge2arrays`
12. Maximum Subarray Sum: `q12_max_subarray_sum`

Each program is located in the `src` directory and can be run using the instructions provided above.

