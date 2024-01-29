use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    input = input.trim().to_string();
    println!("Thank you, {}, and farewell!",input)
    
}