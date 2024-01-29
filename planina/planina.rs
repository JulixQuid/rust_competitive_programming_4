use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let side: u32 = input.trim().parse().unwrap();
    
    println!("{}",((2_u32).pow(side) + 1).pow(2));
}