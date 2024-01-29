use std::io;
fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let tokens: Vec<&str> = input.split_whitespace().collect();
    let numbers: Vec<i32> = tokens.into_iter()
                .map(|s| s.trim().parse().unwrap())
                .collect();
    let result: i32 = (numbers[1]-1)*numbers[0]+1;
    println!("{}",result)
}