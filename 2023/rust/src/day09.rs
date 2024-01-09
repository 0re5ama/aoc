use itertools::Itertools;

pub fn sln(input: String, q: Option<u8>) {
    let parsed: Vec<Vec<i64>> = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect();
    match q {
        Some(1) => println!("{}", q1(&parsed)),
        Some(2) => println!("{}", q2(&parsed)),
        _ => {
            println!("{}", q1(&parsed));
            println!("{}", q2(&parsed));
        }
    }
}

enum Dir {
    Prev,
    Next,
}

fn interpolate(seq: &Vec<i64>, dir: Dir) -> i64 {
    let mut iters: Vec<Vec<i64>> = Vec::new();
    let mut flag = false;
    let mut iter = seq.clone();
    while !flag {
        iter = iter
            .iter()
            .tuple_windows()
            .map(|(curr, next)| next - curr)
            .collect();
        flag = iter.iter().all_equal();
        iters.push(iter.clone());
    }
    let fun = |diff: i64, list: &Vec<i64>| -> i64 {
        match dir {
            Dir::Prev => list[0] - diff,
            Dir::Next => list.iter().last().unwrap() + diff,
        }
    };

    let diff = iters.iter().rev().fold(0i64, fun);
    fun(diff, seq)
}

fn q1(input: &[Vec<i64>]) -> i64 {
    input
        .iter()
        .fold(0, |a, seq| a + interpolate(seq, Dir::Next))
}

fn q2(input: &[Vec<i64>]) -> i64 {
    input
        .iter()
        .fold(0, |a, seq| a + interpolate(seq, Dir::Prev))
}
