use std::collections::HashMap;

use num::Integer;
use regex::Regex;

struct Instr {
    l: String,
    r: String,
}

impl Instr {
    fn new(l: String, r: String) -> Self {
        Self { l, r }
    }
    fn get(&self, dir: &str) -> &str {
        match dir {
            "L" => self.l.as_str(),
            "R" => self.r.as_str(),
            _ => panic!("direction mismatch {}", dir),
        }
    }
}

pub fn sln(input: String, q: Option<u8>) {
    match q {
        Some(1) => println!("{}", q1(&input)),
        Some(2) => println!("{}", q2(&input)),
        _ => {
            println!("{}", q1(&input));
            println!("{}", q2(&input));
        }
    }
}

struct Set {
    map: HashMap<String, Instr>,
    dirs: String,
}

impl Set {
    fn new(map: HashMap<String, Instr>, dirs: String) -> Self {
        Self { map, dirs }
    }
}

fn gen_set(input: &String) -> Set {
    let data = input.split("\n\n").collect::<Vec<&str>>();
    let dirs = data[0].to_string();
    let re = Regex::new(r"\w{3}").unwrap();
    let mut map = HashMap::new();
    data[1].lines().for_each(|l| {
        let vec = re.find_iter(l).map(|c| c.as_str()).collect::<Vec<&str>>();
        map.insert(
            vec[0].to_string(),
            Instr::new(vec[1].to_string(), vec[2].to_string()),
        );
    });

    Set::new(map, dirs)
}

fn q1(input: &String) -> u64 {
    let start = "AAA";
    let set = gen_set(input);
    resolve(&set, start, true)
}

fn q2(input: &String) -> u64 {
    let set = gen_set(input);
    let list = set
        .map
        .keys()
        .filter(|k| Regex::new(r"A$").unwrap().is_match(k))
        .map(|start| resolve(&set, start, false));
    list.into_iter().fold(1, |a, b| a.lcm(&b))
}

fn resolve(set: &Set, start: &str, zzz: bool) -> u64 {
    let re = Regex::new(r"Z$").unwrap();
    let mut found = false;
    let mut curr = start;
    let mut count = 0usize;
    while !found {
        let ix = count as usize % set.dirs.len();
        let dir = set.dirs.get(ix..ix + 1).unwrap();
        curr = set.map.get(curr).unwrap().get(dir);
        count += 1;

        match zzz {
            true => found = curr == "ZZZ",
            false => found = re.is_match(curr),
        }
    }
    count as u64
}
