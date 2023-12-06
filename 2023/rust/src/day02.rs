use std::collections::HashMap;

const RED_MAX: u64 = 12;
const GREEN_MAX: u64 = 13;
const BLUE_MAX: u64 = 14;

#[derive(Default)]
struct Bag {
    red: u64,
    green: u64,
    blue: u64,
}

impl Bag {
    fn is_valid(&self) -> bool {
        self.red <= RED_MAX && self.green <= GREEN_MAX && self.blue <= BLUE_MAX
    }

    fn higher(&self, other: &Self) -> Self {
        Self {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }

    fn power(&self) -> u64 {
        self.red * self.green * self.blue
    }
}

struct Game {
    id: u64,
    bags: Vec<Bag>,
}

pub fn sln(input: String) {
    let games = parse(&input);
    println!("{}", q1(&games));
    println!("{}", q2(&games));
}

fn q1(games: &Vec<Game>) -> u64 {
    games
        .iter()
        .filter(|game| game.bags.iter().all(|bag| bag.is_valid()))
        .map(|game| game.id)
        .sum()
}

fn q2(games: &Vec<Game>) -> u64 {
    games
        .iter()
        .map(|game| game.bags.iter().fold(Bag::default(), |a, g| a.higher(g)))
        .map(|bag| bag.power())
        .sum()
}

fn parse(input: &String) -> Vec<Game> {
    input
        .lines()
        .map(|line| -> Game {
            let mut parts = line.split(": ");
            let game_id = parts
                .next()
                .unwrap_or("")
                .split_whitespace()
                .nth(1)
                .unwrap_or("")
                .parse::<u64>()
                .unwrap_or(0);
            let bags = parts
                .next()
                .unwrap_or("")
                .split(";")
                .map(|bag_str| {
                    let mut map = HashMap::new();
                    bag_str.split(", ").for_each(|cube| {
                        let mut iter = cube.split_whitespace();
                        if let (Some(num_str), Some(color)) = (iter.next(), iter.next()) {
                            if let Ok(num) = num_str.parse::<u64>() {
                                map.insert(color, num);
                            }
                        }
                    });
                    Bag {
                        red: *map.get("red").unwrap_or(&0),
                        blue: *map.get("blue").unwrap_or(&0),
                        green: *map.get("green").unwrap_or(&0),
                    }
                })
                .collect::<Vec<_>>();
            Game { id: game_id, bags }
        })
        .collect()
}
