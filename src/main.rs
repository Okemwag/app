use std::io;

fn main() {
    // Print a welcome message
    println!("Hello! This is a simple Rust program.");

    // Get user input
    println!("Please enter your name:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Print a personalized greeting
    let name = input.trim();
    println!("Nice to meet you, {}! This program has {} lines of code.", name, count_lines());
}

// Function to count the number of lines in the program
fn count_lines() -> usize {
    // Open the source code file and count lines
    let source_code = include_str!("/home/okemwag/app/src/main.rs");
    source_code.lines().count()
}

