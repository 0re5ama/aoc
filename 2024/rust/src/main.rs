mod solutions;
use std::fs;

use solutions::day10::{gold, silver};

fn main() {
    let input = read_file("10");
    let silver = silver(&input);
    let gold = gold(&input);
    println!("silver: {}", silver);
    println!("gold: {}", gold);
}

fn read_file(day: &str) -> String {
    fs::read_to_string(format!("../inputs/{day}")).expect("Unable to read input")
}
