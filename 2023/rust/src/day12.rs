use std::{collections::HashMap, iter};

use indicatif::ProgressIterator;
use itertools::Itertools;

type Memo = HashMap<(Vec<char>, Vec<usize>), usize>;

pub fn sln(input: String, q: Option<u8>) {
    let _ = input;
    match q {
        Some(1) => println!("{}", _q1(&input)),
        Some(2) => println!("{}", q2(&input)),
        _ => {
            println!("{}", q1(&input));
            println!("{}", q2(&input));
        }
    }
}

// brute force
fn _q1(input: &str) -> usize {
    let lavas = parse(input, 1);
    let vec = lavas.iter().progress().map(|l| l._calc());
    vec.sum()
}

fn q1(input: &str) -> usize {
    let lavas = parse(input, 1);
    let vec = lavas.iter().progress().map(|l| l.calc());
    vec.sum()
}

fn q2(input: &str) -> usize {
    let lavas = parse(input, 5);
    let vec = lavas.iter().progress().map(|l| l.calc());
    vec.sum()
}

#[derive(Debug)]
struct Lava {
    pattern: String,
    condition: Vec<usize>,
}

macro_rules! memo_return {
    ($memo:expr, $pat:expr, $cond:expr, $val:expr) => {
        let res = $val;
        $memo.insert(($pat.to_vec(), $cond.to_vec()), res);
        return res;
    };
}

impl Lava {
    fn new(input: &str, n: usize) -> Self {
        let (pattern, condition) = input.split_once(' ').unwrap();
        let pattern = iter::repeat(pattern).take(n).join("?");
        let condition = iter::repeat(condition)
            .take(n)
            .join(",")
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();
        Self { pattern, condition }
    }

    fn calc(&self) -> usize {
        let mut memo = HashMap::new();
        Self::count(
            &self.pattern.chars().collect_vec(),
            &self.condition,
            &mut memo,
        )
    }

    fn _calc(&self) -> usize {
        let unknowns = self
            .pattern
            .chars()
            .enumerate()
            .filter_map(|(i, c)| match c {
                '?' => Some(i),
                _ => None,
            })
            .collect_vec();

        let possibilities = unknowns
            .iter()
            .powerset()
            .filter(|p| {
                let mut str = self.pattern.clone();
                p.iter().for_each(|i| {
                    str.replace_range(**i..*i + 1, "#");
                });
                let str = str.replace('?', ".");
                let res = Lava {
                    pattern: str.clone(),
                    condition: self.condition.clone(),
                }
                .eval();
                res == self.condition
            })
            .collect_vec()
            .len();
        possibilities
    }

    fn count(pat: &[char], cond: &[usize], memo: &mut Memo) -> usize {
        if let Some(res) = memo.get(&(pat.to_vec(), cond.to_vec())) {
            return *res;
        }

        if cond.is_empty() {
            memo_return!(memo, pat, cond, !pat.contains(&'#') as usize);
        }

        if pat.is_empty() {
            memo_return!(memo, pat, cond, cond.is_empty() as usize);
        }

        if pat.len() < cond[0] {
            memo_return!(memo, pat, cond, 0);
        }

        memo_return!(memo, pat, cond, match pat[0] {
            '.' => Self::dot(pat, cond, memo),
            '#' => Self::hash(pat, cond, memo),
            '?' => Self::dot(pat, cond, memo) + Self::hash(pat, cond, memo),
            _ => panic!("muda muda muda muda muda"),
        });
    }

    fn hash(pat: &[char], cond: &[usize], memo: &mut Memo) -> usize {
        if pat.len() < cond[0] || pat[0..cond[0]].contains(&'.') {
            memo_return!(memo, pat, cond, 0);
        }

        if pat.len() == cond[0] {
            memo_return!(memo, pat, cond, (cond.len() == 1) as usize);
        }

        if pat[cond[0]] == '#' {
            memo_return!(memo, pat, cond, 0);
        }

        Self::count(&pat[cond[0] + 1..], &cond[1..], memo)
    }

    fn dot(pat: &[char], cond: &[usize], memo: &mut Memo) -> usize {
        Self::count(&pat[1..], cond, memo)
    }

    fn eval(&self) -> Vec<usize> {
        self.pattern
            .split(['?', '.'])
            .map(|x| x.match_indices('#').collect_vec().len())
            .filter(|x| *x != 0)
            .collect_vec()
    }
}

fn parse(input: &str, n: usize) -> Vec<Lava> {
    input.lines().map(|l| Lava::new(l, n)).collect_vec()
}
