use std::fmt;

#[derive(Debug)]
struct Map {
    dst: u64,
    src: u64,
    cnt: u64,
}

struct Garden {
    seeds: Vec<u64>,
    mappers: Vec<Mapper>,
}

impl Garden {
    fn new(seeds: Vec<u64>, mappers: Vec<Mapper>) -> Self {
        Self { seeds, mappers }
    }
}

#[derive(Debug)]
struct Mapper {
    maps: Vec<Map>,
    name: String,
}

impl fmt::Display for Mapper {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_fmt(format_args!("{}", self.name))
    }
}

impl Mapper {
    fn new(maps: Vec<Map>, name: String) -> Self {
        Self { maps, name }
    }

    fn map_val(&self, input: u64) -> u64 {
        let map = self.maps.iter().find(|m| m.range().contains(&input));
        match map {
            Some(map) => input - map.src + map.dst,
            _ => input,
        }
    }
}

impl Map {
    fn new(dest: u64, src: u64, cnt: u64) -> Self {
        Self {
            dst: dest,
            src,
            cnt,
        }
    }
    fn range(&self) -> std::ops::Range<u64> {
        self.src..(self.src + self.cnt)
    }
}

pub fn sln(input: String) {
    let garden = parse(&input);
    println!("{}", q1(&garden));
    println!("{}", q2(&garden));
}

fn localize(s: u64, garden: &Garden) -> u64 {
    let mut disp = s.to_string();
    let res = garden.mappers.iter().fold(s, |a, m| {
        let val = m.map_val(a);
        disp = format!("{disp} -> {val}");
        val
    });
    disp = format!("{disp} -> {res}");
    println!("{disp}");
    res
}

fn q1(garden: &Garden) -> u64 {
    garden
        .seeds
        .iter()
        .map(|&s| localize(s, garden))
        .min()
        .unwrap()
}

fn q2(garden: &Garden) -> u64 {
    for m in garden.mappers.iter().rev() {
        println!("{m:?}");
    }
    0
}

fn _q2_brute(garden: &Garden) -> u64 {
    let mut min = u64::max_value();
    for i in 0..garden.seeds.len() / 2 {
        let (start, end) = (
            garden.seeds[i * 2],
            garden.seeds[i * 2] + garden.seeds[i * 2 + 1],
        );
        println!("====================================================================================================");
        println!("{start} ==> {end}");
        println!("====================================================================================================");
        for i in start..end {
            min = min.min(localize(i, garden));
        }
    }
    min
}

const MAPPER_NAMES: [&str; 7] = [
    "seed to soil",
    "soil to fertilizer",
    "fertilizer to water",
    "water to light",
    "light to temperature",
    "temperature to humidity",
    "humidity to location",
];

fn parse(input: &String) -> Garden {
    let mut list = input.split("\n\n");
    let seeds_str = list.nth(0).unwrap().split(":").nth(1).unwrap().trim();
    let seeds = seeds_str
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let mut i = 0;
    let mappers = list
        .map(|map_str| {
            i = i + 1;
            let mut maps = map_str
                .split(":")
                .nth(1)
                .unwrap()
                .trim()
                .lines()
                .map(|l| {
                    let arr: Vec<u64> = l
                        .split_whitespace()
                        .map(|n| n.parse::<u64>().unwrap())
                        .collect();
                    let mpr = Map::new(arr[0], arr[1], arr[2]);
                    mpr
                })
                .collect::<Vec<Map>>();
            maps.sort_by(|a, b| a.dst.cmp(&b.dst));
            Mapper::new(maps, String::from(MAPPER_NAMES[i]))
        })
        .collect::<Vec<_>>();

    Garden::new(seeds, mappers)
}
