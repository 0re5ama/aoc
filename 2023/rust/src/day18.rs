use rust::Direction;

pub fn sln(input: String, q: Option<u8>) {
    match q {
        Some(1) => println!("{}", q1(&input)),
        Some(2) => println!("{}", q2()),
        _ => {
            println!("{}", q1(&input));
            println!("{}", q2());
        }
    }
}

struct Operation {
    dir: Direction,
}

fn q1(input: &str) -> u64 {
    let ops = input.lines().map(|l| l);
    0
}

fn q2() -> u64 {
    0
}
