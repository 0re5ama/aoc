use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn sln(input: String, q: Option<u8>) {
    let _ = input;
    match q {
        Some(1) => println!("{}", q1(&input)),
        Some(2) => println!("{}", q2(&input)),
        _ => {
            println!("{}", q1(&input));
            println!("{}", q2(&input));
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct State {
    // TODO: Convert this to a single enum
    pos: Point,
    dir: Direction,
}

#[derive(Debug)]
struct TileInfo {
    tiles: Vec<Vec<char>>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TileInfo {
    fn new(tiles: Vec<Vec<char>>) -> Self {
        Self { tiles }
    }

    fn up(&self, pos: &Point) -> Option<State> {
        if pos.x == 0 {
            None
        } else {
            Some(State {
                pos: Point {
                    x: pos.x - 1,
                    y: pos.y,
                },
                dir: Direction::Up,
            })
        }
    }

    fn down(&self, pos: &Point) -> Option<State> {
        if pos.x == self.tiles.len() - 1 {
            None
        } else {
            Some(State {
                pos: Point {
                    x: pos.x + 1,
                    y: pos.y,
                },
                dir: Direction::Down,
            })
        }
    }

    fn left(&self, pos: &Point) -> Option<State> {
        if pos.y == 0 {
            None
        } else {
            Some(State {
                pos: Point {
                    x: pos.x,
                    y: pos.y - 1,
                },
                dir: Direction::Left,
            })
        }
    }

    fn right(&self, pos: &Point) -> Option<State> {
        if pos.y == self.tiles[0].len() - 1 {
            None
        } else {
            Some(State {
                pos: Point {
                    x: pos.x,
                    y: pos.y + 1,
                },
                dir: Direction::Right,
            })
        }
    }

    fn eval(&self, st: &State) -> usize {
        let mut visited = HashMap::new();
        self.trace(Some(*st), &mut visited);

        let mut pts = HashSet::new();
        for k in visited.keys() {
            pts.insert(k.pos);
        }
        pts.len()
    }

    fn get(&self, pos: &Point) -> char {
        self.tiles[pos.x][pos.y]
    }

    fn trace(&self, st: Option<State>, visited: &mut HashMap<State, bool>) {
        if let Some(st) = st {
            if visited.get(&st).is_some() {
                return;
            }
            visited.insert(st, true);
            match st.dir {
                Direction::Up => match self.get(&st.pos) {
                    '.' | '|' => self.trace(self.up(&st.pos), visited),
                    '-' => {
                        self.trace(self.left(&st.pos), visited);
                        self.trace(self.right(&st.pos), visited);
                    }
                    '\\' => self.trace(self.left(&st.pos), visited),
                    '/' => self.trace(self.right(&st.pos), visited),
                    _ => panic!("muda muda"),
                },
                Direction::Down => match self.get(&st.pos) {
                    '.' | '|' => self.trace(self.down(&st.pos), visited),
                    '-' => {
                        self.trace(self.left(&st.pos), visited);
                        self.trace(self.right(&st.pos), visited);
                    }
                    '\\' => self.trace(self.right(&st.pos), visited),
                    '/' => self.trace(self.left(&st.pos), visited),
                    _ => panic!("muda muda"),
                },
                Direction::Left => match self.get(&st.pos) {
                    '.' | '-' => self.trace(self.left(&st.pos), visited),
                    '|' => {
                        self.trace(self.up(&st.pos), visited);
                        self.trace(self.down(&st.pos), visited);
                    }
                    '\\' => self.trace(self.up(&st.pos), visited),
                    '/' => self.trace(self.down(&st.pos), visited),
                    _ => panic!("muda muda"),
                },
                Direction::Right => match self.get(&st.pos) {
                    '.' | '-' => self.trace(self.right(&st.pos), visited),
                    '|' => {
                        self.trace(self.up(&st.pos), visited);
                        self.trace(self.down(&st.pos), visited);
                    }
                    '\\' => self.trace(self.down(&st.pos), visited),
                    '/' => self.trace(self.up(&st.pos), visited),
                    _ => panic!("muda muda"),
                },
            }
        }
    }
}

fn q1(input: &str) -> usize {
    let info = TileInfo::new(input.lines().map(|l| l.chars().collect_vec()).collect_vec());
    info.eval(&State {
        pos: Point { x: 0, y: 0 },
        dir: Direction::Right,
    })
}

fn q2(input: &str) -> usize {
    let info = TileInfo::new(input.lines().map(|l| l.chars().collect_vec()).collect_vec());
    let rows = info.tiles.len();
    let cols = info.tiles[0].len();

    let row_max = (0..rows)
        .map(|i| {
            let left = State {
                pos: Point { x: i, y: 0 },
                dir: Direction::Right,
            };

            let right = State {
                pos: Point { x: i, y: cols - 1 },
                dir: Direction::Left,
            };

            info.eval(&left).max(info.eval(&right))
        })
        .max()
        .unwrap_or(0);

    let col_max = (0..cols)
        .map(|i| {
            let top = State {
                pos: Point { x: 0, y: i },
                dir: Direction::Down,
            };

            let bot = State {
                pos: Point { x: rows - 1, y: i },
                dir: Direction::Up,
            };

            info.eval(&top).max(info.eval(&bot))
        })
        .max()
        .unwrap_or(0);

    row_max.max(col_max)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let input = r"
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        let res = q1(input.trim());
        assert_eq!(46, res);
    }

    #[test]
    fn test2() {
        let input = r"
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        let res = q2(input.trim());
        assert_eq!(51, res);
    }
}
