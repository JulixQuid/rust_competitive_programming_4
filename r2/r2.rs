use std::io::{self, BufRead};

fn main() {
    let mut input = String::new();

    // Read a line from the standard input

    io::stdin().read_line(&mut input);

    // Split the input into parts

    let parts: Vec<&str> = input.split_whitespace().collect();

    let numbers: Vec<i32> = parts
        .into_iter()
        .map(|s| s.trim().parse().unwrap())
        .collect();
    let solution: i32 = 2*numbers[1]-numbers[0];
    println!("{}",solution)
    }