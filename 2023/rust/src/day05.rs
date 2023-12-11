use std::{fmt, ops::Range};

use itertools::Itertools;

pub fn sln(input: String, q: Option<u8>) {
    let garden = parse(&input);
    match q {
        Some(1) => println!("{}", q1(&garden)),
        Some(2) => println!("{}", q2(&garden)),
        _ => {
            println!("{}", q1(&garden));
            println!("{}", q2(&garden));
        }
    }
}

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
}

impl Map {
    fn new(dest: u64, src: u64, cnt: u64) -> Self {
        Self {
            dst: dest,
            src,
            cnt,
        }
    }
}

struct MapTree {
    src: Range<u64>,
    dst: Range<u64>,
}

impl MapTree {
    fn new(src: Range<u64>, dst: Range<u64>) -> Self {
        Self { src, dst }
    }

    fn map_from_src(&self, src: &Range<u64>) -> Self {
        let end = if src.end == u64::MAX {
            u64::MAX
        } else {
            src.end - self.src.start + self.dst.start
        };
        let start = src.start - self.src.start + self.dst.start;
        let res = Self::new(src.clone(), start..end);
        res
    }

    fn map_from_dst(&self, dst: &Range<u64>) -> Self {
        let end = if dst.end == u64::MAX {
            u64::MAX
        } else {
            self.src.start + dst.end - self.dst.start
        };
        let start = self.src.start + dst.start - self.dst.start;
        let res = Self::new(start..end, dst.clone());
        res
    }
}

impl fmt::Display for MapTree {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_fmt(format_args!("{:?} -> {:?}", self.src, self.dst))
    }
}

impl fmt::Debug for MapTree {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_fmt(format_args!("{:?} -> {:?}", self.src, self.dst))
    }
}

fn intersection<T>(a: &Range<T>, b: &Range<T>) -> Option<Range<T>>
where
    T: Ord,
    T: Copy,
{
    let start = a.start.max(b.start);
    let end = a.end.min(b.end);
    if start <= end {
        Some(start..end)
    } else {
        None
    }
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
            Mapper::new(maps, String::from(MAPPER_NAMES[i - 1]))
        })
        .collect::<Vec<_>>();

    Garden::new(seeds, mappers)
}

fn q1(garden: &Garden) -> u64 {
    solve(garden.seeds.iter().map(|&s| s..s + 1).collect(), garden)
}

fn q2(garden: &Garden) -> u64 {
    solve(
        garden
            .seeds
            .iter()
            .tuples()
            .map(|(a, b)| *a..*a + *b)
            .collect(),
        garden,
    )
}

fn solve(ranges: Vec<Range<u64>>, garden: &Garden) -> u64 {
    let mut all_tree: Vec<MapTree> = Vec::new();
    garden.mappers.iter().rev().for_each(|m| {
        let mut tree: Vec<MapTree> = Vec::new();
        let mut init = 0u64;
        m.maps.iter().for_each(|mm| {
            if mm.dst != init {
                tree.push(MapTree::new(init..mm.dst, init..mm.dst));
            }
            tree.push(MapTree::new(
                mm.src..mm.src + mm.cnt,
                mm.dst..mm.dst + mm.cnt,
            ));
            init = mm.dst + mm.cnt;
        });
        tree.push(MapTree::new(init..u64::MAX, init..u64::MAX));

        if all_tree.len() == 0 {
            all_tree.append(&mut tree);
            return;
        }

        all_tree = tree
            .iter()
            .map(|curr| {
                let valid: Vec<MapTree> = all_tree
                    .iter()
                    .filter_map(|next| match intersection(&next.src, &curr.dst) {
                        Some(r) => Some(MapTree::new(
                            curr.map_from_dst(&r).src,
                            next.map_from_src(&r).dst,
                        )),
                        None => None,
                    })
                    .collect();
                valid
            })
            .flatten()
            .sorted_by(|a, b| a.dst.start.cmp(&b.dst.start))
            .collect();
    });

    ranges
        .iter()
        .map(|range| {
            all_tree
                .iter()
                .filter_map(|next| match intersection(&next.src, range) {
                    Some(r) => Some(next.map_from_src(&r).dst.start),
                    None => None,
                })
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}
