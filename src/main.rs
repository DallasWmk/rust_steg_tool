use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("My path is: {}", args[0]);
    println!("You want to use this text file: {}", args[1]);
}