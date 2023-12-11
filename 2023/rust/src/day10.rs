use std::collections::HashMap;

use enum_iterator::{all, Sequence};
use itertools::Itertools;

pub fn sln(input: String, q: Option<u8>) {
    let maze = parse(&input);
    match q {
        Some(1) => println!("{}", q1(&maze)),
        Some(2) => println!("{}", q2(&maze)),
        _ => {
            println!("{}", q1(&maze));
            println!("{}", q2(&maze));
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Pos {
    x: i32,
    y: i32,
    ch: char,
}

impl Pos {
    fn new(x: i32, y: i32, ch: char) -> Self {
        Self { x, y, ch }
    }
}

#[derive(Sequence, Debug, Clone)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

struct Maze {
    path: Vec<Pos>,
    points: Vec<Vec<Pos>>,
    map: HashMap<(i32, i32), Pos>,
}

impl Maze {
    fn on_path(&self, pt: &Pos) -> bool {
        // self.path.iter().any(|p| p.x == pt.x && p.y == pt.y)
        if let Some(_) = self.map.get(&(pt.x, pt.y)) {
            true
        } else {
            false
        }
    }
}

fn q1(maze: &Maze) -> usize {
    maze.path.len() / 2
}

fn q2(maze: &Maze) -> usize {
    let ss = maze
        .points
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().filter_map(move |(j, pt)| {
                if maze.on_path(&pt) {
                    return None;
                }

                if (0..j)
                    .filter_map(|y| maze.map.get(&(i as i32, y as i32)))
                    .collect_vec()
                    .iter()
                    .filter(|pt| match pt.ch {
                        '|' | 'L' | 'J' => true,
                        _ => false,
                    })
                    .collect_vec()
                    .len()
                    % 2
                    == 1
                {
                    Some(Pos::new(i as i32, j as i32, pt.ch))
                } else {
                    None
                }
            })
        })
        .collect_vec();
    ss.len()
}

fn next(arr: &Vec<Vec<Pos>>, curr: &Pos, dir: &Dir) -> Option<Pos> {
    let (x, y) = match dir {
        Dir::Left => (0, -1),
        Dir::Right => (0, 1),
        Dir::Up => (-1, 0),
        Dir::Down => (1, 0),
    };
    let xx = curr.x + x;
    let yy = curr.y + y;

    if xx < 0 || yy < 0 || xx >= arr.len() as i32 || yy >= arr[0].len() as i32 {
        None
    } else {
        Some(arr[xx as usize][yy as usize])
    }
}

fn parse(input: &String) -> Maze {
    let arr: Vec<_> = input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(|(j, c)| Pos::new(i as i32, j as i32, c))
                .collect::<Vec<Pos>>()
        })
        .collect();

    let start = arr
        .clone()
        .into_iter()
        .find_map(|row| {
            row.iter()
                .find_map(|pos| if pos.ch == 'S' { Some(*pos) } else { None })
        })
        .unwrap();
    let mut curr = start;
    let poss_dirs = all::<Dir>()
        .filter_map(|d| {
            if let Some(next) = next(&arr, &curr, &d) {
                match d {
                    Dir::Left => match next.ch {
                        '-' | 'L' | 'F' => Some(d),
                        _ => None,
                    },
                    Dir::Right => match next.ch {
                        '-' | 'J' | '7' => Some(d),
                        _ => None,
                    },
                    Dir::Up => match next.ch {
                        '|' | '7' | 'F' => Some(d),
                        _ => None,
                    },
                    Dir::Down => match next.ch {
                        '|' | 'J' | 'L' => Some(d),
                        _ => None,
                    },
                }
            } else {
                None
            }
        })
        .collect_vec();

    let actual_start = match (&poss_dirs[0], &poss_dirs[1]) {
        (Dir::Left, Dir::Up) => 'J',
        (Dir::Left, Dir::Down) => '7',
        (Dir::Right, Dir::Up) => 'L',
        (Dir::Right, Dir::Down) => 'F',
        _ => panic!("Masaka!!!"),
    };

    let mut map = HashMap::new();
    let actual_start = Pos {
        ch: actual_start,
        ..curr
    };
    map.insert((curr.x, curr.y), actual_start);
    let mut path: Vec<Pos> = vec![actual_start];

    let mut curr_dir = poss_dirs[0].clone();

    loop {
        let next = next(&arr, &curr, &curr_dir).clone().unwrap();
        if next.ch != 'S' {
            map.insert((next.x, next.y), next);
        }
        path.push(next);
        curr = next;
        curr_dir = match next.ch {
            'S' => {
                break;
            }
            '-' => match curr_dir {
                Dir::Left | Dir::Right => curr_dir,
                _ => panic!("Masaka!!!"),
            },
            '|' => match curr_dir {
                Dir::Up | Dir::Down => curr_dir,
                _ => panic!("Baka Na!!!"),
            },
            'F' => match curr_dir {
                Dir::Left => Dir::Down,
                Dir::Up => Dir::Right,
                _ => panic!("Masaka!!!"),
            },
            '7' => match curr_dir {
                Dir::Right => Dir::Down,
                Dir::Up => Dir::Left,
                _ => panic!("Masaka!!!"),
            },
            'J' => match curr_dir {
                Dir::Right => Dir::Up,
                Dir::Down => Dir::Left,
                _ => panic!("Masaka!!!"),
            },
            'L' => match curr_dir {
                Dir::Left => Dir::Up,
                Dir::Down => Dir::Right,
                _ => panic!("Masaka!!!"),
            },
            _ => panic!("Unpossible!!!"),
        };
    }
    Maze {
        path,
        points: arr,
        map,
    }
}
