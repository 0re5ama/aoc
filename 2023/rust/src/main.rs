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
    let sln_part = match &args.get(2) {
        Some(n) => match n.parse::<u8>() {
            Ok(n) => Some(n),
            _ => None,
        },
        None => None,
    };

    // \d+ -> input
    // \d+t -> test
    // \d+st -> silver test
    // \d+gt -> gold test
    match file.as_str() {
        "1" | "1t" | "1st" | "1gt" => day01::sln(input),
        "2" | "2t" | "2st" | "2gt" => day02::sln(input),
        "3" | "3t" | "3st" | "3gt" => day03::sln(input),
        "4" | "4t" | "4st" | "4gt" => day04::sln(input),
        "5" | "5t" | "5st" | "5gt" => day05::sln(input),
        "6" | "6t" | "6st" | "6gt" => day06::sln(input),
        "7" | "7t" | "7st" | "7gt" => day07::sln(input),
        "8" | "8t" | "8st" | "8gt" => day08::sln(input, sln_part),
        "9" | "9t" | "9st" | "9gt" => day09::sln(input),
        "10" | "10t" | "10st" | "10gt" => day10::sln(input),
        "11" | "11t" | "11st" | "11gt" => day11::sln(input),
        "12" | "12t" | "12st" | "12gt" => day12::sln(input),
        "13" | "13t" | "13st" | "13gt" => day13::sln(input),
        "14" | "14t" | "14st" | "14gt" => day14::sln(input),
        "15" | "15t" | "15st" | "15gt" => day15::sln(input),
        "16" | "16t" | "16st" | "16gt" => day16::sln(input),
        "17" | "17t" | "17st" | "17gt" => day17::sln(input),
        "18" | "18t" | "18st" | "18gt" => day18::sln(input),
        "19" | "19t" | "19st" | "19gt" => day19::sln(input),
        "20" | "20t" | "20st" | "20gt" => day20::sln(input),
        "21" | "21t" | "21st" | "21gt" => day21::sln(input),
        "22" | "22t" | "22st" | "22gt" => day22::sln(input),
        "23" | "23t" | "23st" | "23gt" => day23::sln(input),
        "24" | "24t" | "24st" | "24gt" => day24::sln(input),
        "25" | "25t" | "25st" | "25gt" => day25::sln(input),
        &_ => return,
    }

    let elapsed_time = now.elapsed();
    println!("Running took {} milliseconds.", elapsed_time.as_millis());
}

fn read_file(day: &String) -> String {
    fs::read_to_string(format!("inputs/{day}.txt")).expect("Unable to read")
}
