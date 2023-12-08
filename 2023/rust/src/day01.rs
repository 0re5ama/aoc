use fancy_regex::Regex;
use std::collections::HashMap;

pub fn sln(input: String) {
    println!("{}", q1(&input));
    println!("{}", q2(input));
}

fn q1(input: &String) -> u64 {
    let re = r"\d";
    parse(&input, re, re)
}

pub fn q2(input: String) -> u64 {
    let re = r"([0-9]|zero|one|two|three|four|five|six|seven|eight|nine)";
    let er = r"([0-9]|orez|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)";
    parse(&input, re, er)
}

fn parse(input: &String, re: &str, er: &str) -> u64 {
    let nums = HashMap::from([
        ("zero", "0"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    let reg = Regex::new(re).unwrap();
    let rev = Regex::new(er).unwrap();

    input
        .lines()
        .map(|c| -> u64 {
            if let Ok(fd) = reg.find(c).map(|m| m.expect("ERROR").as_str()) {
                let first = nums.get(fd).unwrap_or(&fd);
                let reverse = c.chars().rev().collect::<String>();
                if let Ok(fd_rev) = rev.find(reverse.as_str()).map(|m| m.expect("ERROR").as_str()) {
                    let revnum = fd_rev.chars().rev().collect::<String>();
                    let revnum = &revnum.as_str();
                    let last = nums.get(revnum).unwrap_or(&revnum);
                    return format!("{}{}", first, last).parse::<u64>().unwrap_or(0);
                }
            }
            0
        })
        .sum::<u64>()
}
