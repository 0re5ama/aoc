use std::collections::HashMap;

use regex::Regex;

pub fn sln(input: String, q: Option<u8>) {
    let data = parse(&input);
    match q {
        Some(1) => println!("{}", q1(&data)),
        Some(2) => println!("{}", q2(&data)),
        _ => {
            println!("{}", q1(&data));
            println!("{}", q2(&data));
        }
    }
}

#[derive(Debug)]
struct ScratchCard {
    id: usize,
    card: Vec<u8>,
    win: Vec<u8>,
}

impl ScratchCard {
    fn new(id: usize, card: Vec<u8>, win: Vec<u8>) -> Self {
        Self { id, card, win }
    }

    fn matched(&self) -> usize {
        self.card
            .clone()
            .into_iter()
            .filter(|c| {
                let found = self.win.iter().any(|w| w == c);
                found
            })
            .collect::<Vec<u8>>()
            .len()
    }
}

fn q1(input: &[ScratchCard]) -> u64 {
    input.iter().fold(0, |a, s| {
        let matched = s.matched();
        a + match matched {
            0 => 0,
            _ => 2u64.pow(matched as u32 - 1),
        }
    })
}

fn q2(input: &Vec<ScratchCard>) -> u64 {
    let mut map: HashMap<usize, usize> = HashMap::new();
    input.iter().for_each(|s| {
        let matched = s.matched();
        if let 1.. = matched {
            let count = map.get(&s.id).unwrap_or(&0) + 1;
            (s.id + 1..s.id + 1 + matched).for_each(|m| {
                map.insert(m, map.get(&m).unwrap_or(&0usize) + count);
            });
        }
    });
    map.keys()
        .map(|k| map.get(k).unwrap())
        .fold(0, |a, b| a + *b as u64)
        + input.len() as u64
}

fn parse(input: &str) -> Vec<ScratchCard> {
    let re = Regex::new(r"^Card\s+(?<id>\d+):\s+(?<win>(?:\d+\s+)+)\|\s+(?<card>(?:\d+\s+)+\d+)$")
        .unwrap();
    input
        .lines()
        .map(|l| {
            let caps = re.captures(l).unwrap();
            ScratchCard::new(
                caps.name("id").map_or(0, |n| n.as_str().parse().unwrap()),
                caps.name("card").map_or(Vec::new(), |s| {
                    s.as_str()
                        .split_whitespace()
                        .map(|n| n.parse().unwrap())
                        .collect()
                }),
                caps.name("win").map_or(Vec::new(), |s| {
                    s.as_str()
                        .split_whitespace()
                        .map(|n| n.parse().unwrap())
                        .collect()
                }),
            )
        })
        .collect()
}
