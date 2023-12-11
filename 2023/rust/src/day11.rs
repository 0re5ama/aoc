use std::fmt;

use itertools::Itertools;

pub fn sln(input: String, q: Option<u8>) {
    let space = parse(&input);
    match q {
        Some(1) => println!("{}", q1(&space)),
        Some(2) => println!("{}", q2(&space)),
        _ => {
            println!("{}", q1(&space));
            println!("{}", q2(&space));
        }
    }
}

#[derive(Debug)]
struct Pos {
    x: usize,
    y: usize,
}

impl fmt::Display for Pos {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_fmt(format_args!("({}, {})", self.x, self.y))
    }
}

impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn dist(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Debug)]
struct Space {
    galaxies: Vec<Pos>,
    space_rows: Vec<usize>,
    space_cols: Vec<usize>,
}

impl Space {
    fn new(galaxies: Vec<Pos>, space_rows: Vec<usize>, space_cols: Vec<usize>) -> Self {
        Self {
            galaxies,
            space_rows,
            space_cols,
        }
    }

    fn count_empty(&self, g1: &Pos, g2: &Pos) -> usize {
        self.space_rows
            .iter()
            .filter(|r| (g1.x.min(g2.x)..g1.x.max(g2.x) + 1).contains(r))
            .collect::<Vec<&usize>>()
            .len()
            + self
                .space_cols
                .iter()
                .filter(|r| (g1.y.min(g2.y)..g1.y.max(g2.y) + 1).contains(r))
                .collect::<Vec<&usize>>()
                .len()
    }

    fn solve(&self, size: usize) -> usize {
        self.galaxies
            .iter()
            .tuple_combinations()
            .fold(0, |a, (g1, g2)| {
                a + g1.dist(g2) + (size - 1) * self.count_empty(g1, g2)
            })
    }
}

fn q1(space: &Space) -> usize {
    space.solve(2)
}

fn q2(space: &Space) -> usize {
    space.solve(1_000_000)
}

fn parse(input: &String) -> Space {
    let lines: Vec<String> = input.lines().map(|l| l.to_string()).collect();
    let space_rows: Vec<_> = lines
        .iter()
        .enumerate()
        .filter_map(|(i, l)| match l.chars().any(|c| c == '#') {
            true => None,
            false => Some(i),
        })
        .collect();
    let space_cols: Vec<_> = lines[0]
        .chars()
        .enumerate()
        .filter_map(
            |(i, _)| match lines.iter().all(|l| l.chars().nth(i).unwrap() == '.') {
                true => Some(i),
                false => None,
            },
        )
        .collect();

    let galaxies: Vec<_> = lines
        .iter()
        .enumerate()
        .flat_map(|(i, l)| {
            l.chars().enumerate().filter_map(move |(j, c)| {
                if c == '#' {
                    Some(Pos::new(i.clone(), j.clone()))
                } else {
                    None
                }
            })
        })
        .collect();

    Space::new(galaxies, space_rows, space_cols)
}
