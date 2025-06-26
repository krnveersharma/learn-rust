mod utils;

use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter number for which u want to find fibonnaci");
    io::stdin().read_line(&mut input).unwrap();
    let inp: u32 = input.trim().parse().expect("Please enter a valid number");

    let ans: u64 = utils::fibonacci(inp);
    println!("Fibonacci is {}", ans);
}
