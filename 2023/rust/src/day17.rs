use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    fmt,
};

use enum_iterator::{all, Sequence};
use itertools::Itertools;

pub fn sln(input: String, q: Option<u8>) {
    match q {
        Some(1) => println!("{}", q1(&input, None)),
        Some(2) => println!("{}", q2(&input, None)),
        _ => {
            println!("{}", q1(&input, None));
            println!("{}", q2(&input, None));
        }
    }
}

#[derive(Sequence, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn rev(&self) -> Self {
        match self {
            Direction::Up => Self::Down,
            Direction::Down => Self::Up,
            Direction::Left => Self::Right,
            Direction::Right => Self::Left,
        }
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let dir = if let Some(d) = self.dir {
            format!("{:?}", d)
        } else {
            String::from("*")
        };
        write!(
            f,
            "{} {:?} {} [{}]",
            dir, self.pos, self.dir_count, self.cost
        )
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl fmt::Debug for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Direction::Up => write!(f, "↑"),
            Direction::Down => write!(f, "↓"),
            Direction::Left => write!(f, "←"),
            Direction::Right => write!(f, "→"),
        }
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

macro_rules! p {
    ($x: expr, $y: expr) => {
        Point::new($x, $y)
    };
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct VisNode {
    pos: Point,
    dir: Option<Direction>,
    dir_count: usize,
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct Node {
    pos: Point,
    dir: Option<Direction>,
    cost: usize,
    dir_count: usize,
}

impl Node {
    fn start(pos: &Point) -> Self {
        Self {
            pos: *pos,
            dir: None,
            cost: 0,
            dir_count: 0,
        }
    }

    fn vis(&self) -> VisNode {
        VisNode {
            pos: self.pos,
            dir: self.dir,
            dir_count: self.dir_count,
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

trait Crucible {
    // TODO: Can be made generic
    fn seek(&self, node: &Node, dir: &Direction) -> Option<Node>;
    fn min(&self) -> usize;
    fn max(&self) -> usize;

    fn best(&self, src: &Point, dst: &Point) -> usize {
        let mut queue = BinaryHeap::new();
        queue.push(Node::start(src));
        let mut visited = HashSet::new();

        while let Some(node) = queue.pop() {
            if &node.pos == dst && node.dir_count >= self.min() {
                return node.cost;
            }

            if visited.get(&node.vis()).is_some() {
                continue;
            }
            visited.insert(node.vis());

            all::<Direction>()
                .clone()
                .filter_map(|dir| self.seek(&node, &dir))
                .for_each(|n| {
                    queue.push(n);
                });
        }
        0
    }
}

struct UltraCruc {
    pool: Vec<Vec<usize>>,
}

impl Crucible for UltraCruc {
    fn seek(&self, node: &Node, dir: &Direction) -> Option<Node> {
        if node.dir.is_some()
            && (Some(dir.rev()) == node.dir
                || (node.dir_count >= self.max() && Some(*dir) == node.dir)
                || (node.dir_count < self.min() && Some(*dir) != node.dir))
        {
            return None;
        }

        let pos = node.pos;
        let next = match dir {
            Direction::Up if pos.x != 0 => Some(p!(pos.x - 1, pos.y)),
            Direction::Down if pos.x != self.pool.len() - 1 => Some(p!(pos.x + 1, pos.y)),
            Direction::Left if pos.y != 0 => Some(p!(pos.x, pos.y - 1)),
            Direction::Right if pos.y != self.pool[0].len() - 1 => Some(p!(pos.x, pos.y + 1)),
            _ => None,
        };

        next.map(|pt| Node {
            pos: pt,
            cost: node.cost + self.pool[pt.x][pt.y],
            dir: Some(*dir),
            dir_count: if Some(*dir) == node.dir {
                node.dir_count + 1
            } else {
                1
            },
        })
    }

    fn min(&self) -> usize {
        4
    }

    fn max(&self) -> usize {
        10
    }
}

impl UltraCruc {
    fn new(pool: Vec<Vec<usize>>) -> Self {
        Self { pool }
    }
}

struct MiniCruc {
    pool: Vec<Vec<usize>>,
}

impl Crucible for MiniCruc {
    fn seek(&self, node: &Node, dir: &Direction) -> Option<Node> {
        if Some(dir.rev()) == node.dir || (node.dir_count >= 3 && Some(*dir) == node.dir) {
            return None;
        }

        let pos = node.pos;
        let next = match dir {
            Direction::Up if pos.x != 0 => Some(p!(pos.x - 1, pos.y)),
            Direction::Down if pos.x != self.pool.len() - 1 => Some(p!(pos.x + 1, pos.y)),
            Direction::Left if pos.y != 0 => Some(p!(pos.x, pos.y - 1)),
            Direction::Right if pos.y != self.pool[0].len() - 1 => Some(p!(pos.x, pos.y + 1)),
            _ => None,
        };

        next.map(|pt| Node {
            pos: pt,
            cost: node.cost + self.pool[pt.x][pt.y],
            dir: Some(*dir),
            dir_count: if Some(*dir) == node.dir {
                node.dir_count + 1
            } else {
                1
            },
        })
    }
    fn min(&self) -> usize {
        1
    }

    fn max(&self) -> usize {
        3
    }
}

impl MiniCruc {
    fn new(pool: Vec<Vec<usize>>) -> Self {
        Self { pool }
    }
}

fn q1(input: &str, dest: Option<&Point>) -> usize {
    let data = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap_or(0) as usize)
                .collect_vec()
        })
        .collect_vec();
    let crucible = MiniCruc::new(data);
    let point = p!(crucible.pool.len() - 1, crucible.pool[0].len() - 1);
    let dest = dest.unwrap_or(&point);
    crucible.best(&p!(0, 0), dest)
}

fn q2(input: &str, dest: Option<&Point>) -> usize {
    let data = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap_or(0) as usize)
                .collect_vec()
        })
        .collect_vec();
    let crucible = UltraCruc::new(data);
    let point = p!(crucible.pool.len() - 1, crucible.pool[0].len() - 1);
    // let point = p!(0, 6);
    let dest = dest.unwrap_or(&point);
    crucible.best(&p!(0, 0), dest)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let input = r"
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";
        // let res = q1(input.trim(), Some(&p!(0, 6)));
        let res = q1(input.trim(), None);
        assert_eq!(102, res);
    }

    #[test]
    fn test2() {
        let input = r"
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";
        let res = q2(input.trim(), None);
        assert_eq!(94, res);
    }

    #[test]
    fn test3() {
        let input = r"
111111111111
999999999991
999999999991
999999999991
999999999991
";
        let res = q2(input.trim(), None);
        assert_eq!(71, res);
    }
}
