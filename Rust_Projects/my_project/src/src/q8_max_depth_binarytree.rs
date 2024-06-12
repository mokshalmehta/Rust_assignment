//Question 8: Given a binary tree, implement a function that returns the maximum depth of the tree.

use std::io;

// Definition of a binary tree node
#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }
}

pub fn run() {
    println!("Enter the elements of the binary tree in pre-order traversal (use 'null' for empty nodes):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let mut iter = input.trim().split_whitespace().peekable();
    let root = build_tree(&mut iter);

    match root {
        Some(root) => {
            let depth = max_depth(&root);
            println!("The maximum depth of the binary tree is: {}", depth);
        }
        None => println!("The binary tree is empty"),
    }
}

fn build_tree(iter: &mut std::iter::Peekable<std::str::SplitWhitespace>) -> Option<Box<TreeNode>> {
    if let Some(val_str) = iter.next() {
        if val_str == "null" {
            return None;
        }
        let val: i32 = val_str.parse().expect("Invalid input. Please enter valid integers or 'null'");

        let mut node = TreeNode::new(val);
        node.left = build_tree(iter);
        node.right = build_tree(iter);
        Some(Box::new(node))
    } else {
        None
    }
}

fn max_depth(root: &TreeNode) -> i32 {
    match (root.left.as_ref(), root.right.as_ref()) {
        (Some(left), Some(right)) => 1 + std::cmp::max(max_depth(left), max_depth(right)),
        (Some(left), None) => 1 + max_depth(left),
        (None, Some(right)) => 1 + max_depth(right),
        (None, None) => 1,
    }
}
