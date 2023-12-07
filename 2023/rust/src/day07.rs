use std::cmp::Ordering;

use fancy_regex::Regex;

#[derive(Debug, Clone)]
struct Hand {
    _name: String,
    patt: Regex,
    val: u64,
    val_type: ValType,
}

#[derive(Clone, Debug)]
enum ValType {
    Five,
    Four,
    FullHouse,
    Three,
    Two,
    Pair,
    High,
}

impl Hand {
    fn get_match(&self, str: String) -> Vec<String> {
        let mut hd_str = str.chars().map(|c| c).collect::<Vec<char>>();
        hd_str.sort_by(|a, b| joker_cmp(&a.to_string(), &b.to_string()));
        let hd_str = hd_str.into_iter().collect::<String>();
        match self.patt.captures(&hd_str).unwrap() {
            Some(cap) => cap
                .iter()
                .map(|c| match c {
                    Some(ch) => ch.as_str().to_string(),
                    None => String::from(""),
                })
                .collect::<Vec<String>>(),
            None => Vec::new(),
        }
    }

    fn map_joker(&self, input: String) -> Option<u64> {
        if let None = input.find("J") {
            return None;
        }
        Some(match self.val_type {
            ValType::Five => 7000,
            ValType::Four => 7000,
            ValType::FullHouse => 7000,
            ValType::Three => 6000,
            ValType::Two => match self
                .get_match(input)
                .iter()
                .skip(1)
                .find(|y| y.as_str() == "J")
            {
                Some(_) => 6000,
                None => 5000,
            },
            ValType::Pair => 4000,
            ValType::High => 2000,
        })
    }
}

fn hand_vals() -> [Hand; 7] {
    [
        Hand {
            val_type: ValType::Five,
            _name: String::from("Five of a kind"),
            patt: Regex::new(r"(.)\1{4}").unwrap(),
            val: 7000,
        },
        Hand {
            val_type: ValType::Four,
            _name: String::from("Four of a kind"),
            patt: Regex::new(r"(.)\1{3}").unwrap(),
            val: 6000,
        },
        Hand {
            val_type: ValType::FullHouse,
            _name: String::from("Full house"),
            patt: Regex::new(r"(.)\1(.)\2{2}|(.)\3{2}(.)\4").unwrap(),
            val: 5000,
        },
        Hand {
            val_type: ValType::Three,
            _name: String::from("Three of a kind"),
            patt: Regex::new(r"(.)\1{2}").unwrap(),
            val: 4000,
        },
        Hand {
            val_type: ValType::Two,
            _name: String::from("Two pair"),
            patt: Regex::new(r"(.)\1.(.)\2|(.)\3(.)\4.|.(.)\5(.)\6").unwrap(),
            val: 3000,
        },
        Hand {
            val_type: ValType::Pair,
            _name: String::from("One pair"),
            patt: Regex::new(r"(.)\1").unwrap(),
            val: 2000,
        },
        Hand {
            val_type: ValType::High,
            _name: String::from("High card"),
            patt: Regex::new(r".").unwrap(),
            val: 1000,
        },
    ]
}

#[derive(Debug)]
struct GameHand {
    hand: Hand,
    bet: u64,
    hd_str: String,
}

#[derive(Debug)]
enum Game {
    Normal(GameHand),
    Joker(GameHand),
}

fn normal_cmp(a: &str, b: &str) -> Ordering {
    let order = "AKQJT98765432";
    cmp_hands(a, b, order)
}

fn joker_cmp(a: &str, b: &str) -> Ordering {
    let order = "AKQT98765432J";
    cmp_hands(a, b, order)
}

fn cmp_hands(a: &str, b: &str, order: &str) -> Ordering {
    a.chars()
        .zip(b.chars())
        .find_map(|(x, y)| {
            let a = order.chars().position(|c| c == x).unwrap();
            let b = order.chars().position(|c| c == y).unwrap();
            match a.cmp(&b) {
                Ordering::Equal => None,
                h => Some(h),
            }
        })
        .unwrap_or(Ordering::Equal)
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Game {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Game::Normal(a) => {
                let Game::Normal(b) = other else {
                    panic!("err");
                };
                match a.hand.val.cmp(&b.hand.val) {
                    Ordering::Equal => normal_cmp(b.hd_str.as_str(), a.hd_str.as_str()),
                    Ordering::Less => Ordering::Less,
                    Ordering::Greater => Ordering::Greater,
                }
            }
            Game::Joker(a) => {
                let Game::Joker(b) = other else {
                    panic!("err");
                };

                let a_val = a.hand.val;
                let a_val = a.hand.map_joker(a.hd_str.clone()).unwrap_or(a_val);

                if a.hd_str == String::from("JJ488") {}

                let b_val = b.hand.val;
                let b_val = b.hand.map_joker(b.hd_str.clone()).unwrap_or(b_val);
                match a_val.cmp(&b_val) {
                    Ordering::Equal => joker_cmp(b.hd_str.as_str(), a.hd_str.as_str()),
                    Ordering::Less => Ordering::Less,
                    Ordering::Greater => Ordering::Greater,
                }
            }
        }
    }
}

impl Eq for Game {}

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        match self.cmp(other) {
            Ordering::Equal => true,
            _ => false,
        }
    }
}

pub fn sln(input: String) {
    println!("{}", q1(&input));
    println!("{}", q2(&input));
}

fn q1(input: &String) -> u64 {
    let vals = hand_vals();
    let mut hands = input
        .lines()
        .map(|x| {
            let row = x.split_whitespace().collect::<Vec<&str>>();
            let mut hd_str = row[0].chars().map(|c| c).collect::<Vec<char>>();
            hd_str.sort_by(|a, b| normal_cmp(&a.to_string(), &b.to_string()));
            let hd_str = hd_str.into_iter().collect::<String>();
            let bet = row[1].parse::<u64>().unwrap();
            let hand = vals
                .iter()
                .find(|h| h.patt.is_match(hd_str.as_str()).unwrap_or(false))
                .unwrap();
            Game::Normal(GameHand {
                hand: hand.clone(),
                bet,
                hd_str: row[0].to_string(),
            })
        })
        .collect::<Vec<Game>>();
    hands.sort();
    hands.into_iter().enumerate().fold(0, |a, (i, x)| {
        let Game::Normal(game) = x else {
            panic!("err parsing as normal game");
        };
        a + game.bet * (i + 1) as u64
    })
}

fn q2(input: &String) -> u64 {
    let vals = hand_vals();
    let mut hands = input
        .lines()
        .map(|x| {
            let row = x.split_whitespace().collect::<Vec<&str>>();
            let mut hd_str = row[0].chars().map(|c| c).collect::<Vec<char>>();
            hd_str.sort_by(|a, b| joker_cmp(&a.to_string(), &b.to_string()));
            let hd_str = hd_str.into_iter().collect::<String>();
            let bet = row[1].parse::<u64>().unwrap();
            let hand = vals
                .iter()
                .find(|h| h.patt.is_match(hd_str.as_str()).unwrap_or(false))
                .unwrap();
            Game::Joker(GameHand {
                hand: hand.clone(),
                bet,
                hd_str: row[0].to_string(),
            })
        })
        .collect::<Vec<Game>>();
    hands.sort();
    hands.into_iter().enumerate().fold(0, |a, (i, x)| {
        let Game::Joker(game) = x else {
            panic!("err parsing as normal game");
        };
        a + game.bet * (i + 1) as u64
    })
}
