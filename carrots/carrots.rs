use std::io::{self, BufRead};

fn main() {
    let mut input = String::new();
    let mut input2 = String::new();

    // Read a line from the standard input
    io::stdin().read_line(&mut input);

    // Split the input into parts
    let parts: Vec<&str> = input.split_whitespace().collect();

    // Convert the first element of the collection to an integer
    let iterations = parts[0].trim().parse::<usize>().unwrap();

    // Read lines based on the specified number of iterations
    for _ in 0..iterations {
        io::stdin().read_line(&mut input2);
    }
    println!("{}",parts[1])
    }
