use std::collections::HashMap;

use enum_iterator::Sequence;
use indicatif::ProgressIterator;
use itertools::Itertools;

pub fn sln(input: String, q: Option<u8>) {
    let parsed = input.lines().map(|l| l.chars().collect_vec()).collect_vec();

    match q {
        Some(1) => println!("{}", q1(&parsed)),
        Some(2) => println!("{}", q2(&parsed)),
        _ => {
            println!("{}", q1(&parsed));
            println!("{}", q2(&parsed));
        }
    }
}

fn _disp(dish: &[Vec<char>]) {
    let str = dish.iter().map(|l| l.iter().join("")).join("\n");
    println!("{}", str);
}

fn calc(input: &str) -> usize {
    let lines = input.lines().collect_vec();
    let len = lines.len();
    (0..len)
        .rev()
        .map(|i| {
            let l = lines[i];
            let res = l.chars().filter(|c| c == &'O').collect_vec().len() * (len - i);
            // println!("{}: {} -> {}", i, l, res);
            res
        })
        .sum()
}

fn cycle(input: &[Vec<char>], tot: usize) -> String {
    let mut vec: Vec<String> = Vec::new();
    let mut curr = input.to_owned();
    for i in (0..tot).progress() {
        curr = tilt(&curr, Dir::North);
        curr = tilt(&curr, Dir::West);
        curr = tilt(&curr, Dir::South);
        curr = tilt(&curr, Dir::East);
        let str = curr.iter().map(|l| l.iter().join("")).join("\n");
        if let Some((ix, _)) = vec.iter().find_position(|x| x == &&str) {
            let repeat = i - ix;
            let index = ((&tot - 1 - ix) % repeat) + ix;
            return vec[index].clone();
        } else {
            vec.push(str);
        }
    }
    curr.iter().map(|l| l.iter().join("")).join("\n")
}

fn q1(input: &Vec<Vec<char>>) -> usize {
    let tilted = tilt(input, Dir::North);
    let str = tilted.iter().map(|l| l.iter().join("")).join("\n");
    calc(&str)
}

fn q2(input: &[Vec<char>]) -> usize {
    let tot = 1_000_000_000;
    let val = cycle(input, tot);
    calc(&val)
}

#[derive(Sequence, Debug, Clone)]
enum Dir {
    North,
    West,
    South,
    East,
}

fn tilt(arr: &Vec<Vec<char>>, dir: Dir) -> Vec<Vec<char>> {
    let mut map = HashMap::new();
    match dir {
        Dir::North => arr
            .iter()
            .enumerate()
            .map(|(i, x)| {
                x.iter()
                    .enumerate()
                    .map(|(j, y)| {
                        let yy = map.get(&(i, j)).unwrap_or(y);
                        match yy {
                            'O' | '#' => *yy,
                            '.' => {
                                for (k, _) in arr.iter().enumerate().skip(i + 1) {
                                    match map.get(&(k, j)).unwrap_or(&arr[k][j]) {
                                        '#' => {
                                            return *yy;
                                        }
                                        'O' => {
                                            map.insert((k, j), '.');
                                            map.insert((i, j), 'O');
                                            return 'O';
                                        }
                                        _ => {}
                                    }
                                }
                                *yy
                            }
                            _ => panic!("masaka!"),
                        }
                    })
                    .collect_vec()
            })
            .collect_vec(),
        Dir::West => arr
            .iter()
            .enumerate()
            .map(|(i, x)| {
                x.iter()
                    .enumerate()
                    .map(|(j, y)| {
                        let yy = map.get(&(i, j)).unwrap_or(y);
                        match yy {
                            'O' | '#' => *yy,
                            '.' => {
                                for k in j + 1..x.len() {
                                    match map.get(&(i, k)).unwrap_or(&arr[i][k]) {
                                        '#' => {
                                            return *yy;
                                        }
                                        'O' => {
                                            map.insert((i, k), '.');
                                            map.insert((i, j), 'O');
                                            return 'O';
                                        }
                                        _ => {}
                                    }
                                }
                                *yy
                            }
                            _ => panic!("masaka!"),
                        }
                    })
                    .collect_vec()
            })
            .collect_vec(),
        Dir::South => {
            let mut res = (0..arr.len())
                .rev()
                .map(|i| {
                    let x = &arr[i];
                    // println!("{i}, {:?}", x.iter().join(""));
                    let res = x
                        .iter()
                        .enumerate()
                        .map(|(j, y)| {
                            let yy = map.get(&(i, j)).unwrap_or(y);
                            match yy {
                                'O' | '#' => *yy,
                                '.' => {
                                    for k in (0..i).rev() {
                                        // println!("({}, {}): {}  {}: {}", i, j, yy, k, arr[k][j]);
                                        match map.get(&(k, j)).unwrap_or(&arr[k][j]) {
                                            '#' => {
                                                return *yy;
                                            }
                                            'O' => {
                                                map.insert((k, j), '.');
                                                map.insert((i, j), 'O');
                                                return 'O';
                                            }
                                            _ => {}
                                        }
                                    }
                                    *yy
                                }
                                _ => panic!("masaka!"),
                            }
                        })
                        .collect_vec();
                    // println!("{}: {:?}", i, res.iter().join(""));
                    res
                })
                .collect_vec();
            res.reverse();
            res
        }
        Dir::East => arr
            .iter()
            .enumerate()
            .map(|(i, x)| {
                let mut res = (0..x.len())
                    .rev()
                    .map(|j| {
                        let y = x[j];
                        let yy = map.get(&(i, j)).unwrap_or(&y);
                        match yy {
                            'O' | '#' => *yy,
                            '.' => {
                                for k in (0..j).rev() {
                                    match map.get(&(i, k)).unwrap_or(&arr[i][k]) {
                                        '#' => {
                                            return *yy;
                                        }
                                        'O' => {
                                            map.insert((i, k), '.');
                                            map.insert((i, j), 'O');
                                            return 'O';
                                        }
                                        _ => {}
                                    }
                                }
                                *yy
                            }
                            _ => panic!("masaka!"),
                        }
                    })
                    .collect_vec();
                res.reverse();
                res
            })
            .collect_vec(),
    }
}
