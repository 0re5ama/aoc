mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

use std::env::args;
use std::fs;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let args: Vec<_> = args().collect();
    let file = &args[1];
    let input = read_file(file);

    match file.as_str() {
        "1" | "1t" => day01::sln(input),
        "2" | "2t" => day02::sln(input),
        "3" | "3t" => day03::sln(input),
        "4" | "4t" => day04::sln(input),
        "5" | "5t" => day05::sln(input),
        "6" | "6t" => day06::sln(input),
        "7" | "7t" => day07::sln(input),
        "8" | "8t" => day08::sln(input),
        "9" | "9t" => day09::sln(input),
        "10" | "10t" => day10::sln(input),
        "11" | "11t" => day11::sln(input),
        "12" | "12t" => day12::sln(input),
        "13" | "13t" => day13::sln(input),
        "14" | "14t" => day14::sln(input),
        "15" | "15t" => day15::sln(input),
        "16" | "16t" => day16::sln(input),
        "17" | "17t" => day17::sln(input),
        "18" | "18t" => day18::sln(input),
        "19" | "19t" => day19::sln(input),
        "20" | "20t" => day20::sln(input),
        "21" | "21t" => day21::sln(input),
        "22" | "22t" => day22::sln(input),
        "23" | "23t" => day23::sln(input),
        "24" | "24t" => day24::sln(input),
        "25" | "25t" => day25::sln(input),
        &_ => return,
    }

    let elapsed_time = now.elapsed();
    println!("Running took {} milliseconds.", elapsed_time.as_millis());
}

fn read_file(day: &String) -> String {
    fs::read_to_string(format!("inputs/{day}.txt")).expect("Unable to read")
}
