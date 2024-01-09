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

fn single_smudge(lhs: &[&String], rhs: &[&String]) -> bool {
    let temp_l = lhs.iter().join("");
    let temp_r = rhs.iter().join("");
    let mut iter = temp_l.chars().zip(temp_r.chars()).filter(|(l, r)| l != r);
    iter.next().is_some() && iter.next().is_none()
}

fn calc(lines: &[String], part: u8) -> usize {
    for i in 1..lines.len() {
        let lhs = lines.iter().take(i);
        let rhs = lines.iter().skip(i).rev();
        let lhs_len = lhs.len();
        let rhs_len = rhs.len();
        let mut lhs_vec = lhs.collect_vec();
        let mut rhs_vec = rhs.collect_vec();

        if rhs_len < lhs_len {
            lhs_vec = lhs_vec
                .iter()
                .skip(lhs_len - rhs_len)
                .copied()
                .collect_vec();
        } else {
            rhs_vec = rhs_vec
                .iter()
                .skip(rhs_len - lhs_len)
                .copied()
                .collect_vec();
        }

        #[cfg(test)]
        if part == 2 && lhs_len == 4 {
            println!(
                "{}: {}, {}\n{:#?} => {:#?}\n{}",
                match part {
                    1 => "row",
                    2 => "col",
                    _ => panic!("muda muda muda"),
                },
                lhs_len,
                rhs_len,
                lhs_vec,
                rhs_vec,
                lhs_vec == rhs_vec,
            );
        }

        if part == 1 && lhs_vec == rhs_vec {
            return i;
        }

        if part == 2 && single_smudge(&lhs_vec, &rhs_vec) {
            return i;
        }
    }
    0
}

fn get_row(lines: &[&str], part: u8) -> usize {
    calc(&lines.iter().map(|x| x.to_string()).collect_vec(), part) * 100
}

fn get_col(lines: &[&str], part: u8) -> usize {
    let cols = lines[0]
        .chars()
        .enumerate()
        .map(|(i, _)| lines.iter().map(move |l| l.as_bytes()[i] as char).join(""))
        .collect_vec();
    calc(&cols, part)
}

fn q1(input: &str) -> usize {
    let lines = input.split("\n\n").map(|pat| {
        let patt = pat.lines().collect_vec();
        let row = get_row(&patt, 1);
        let col = get_col(&patt, 1);
        if row == 0 && col == 0 {
            println!("{:?} => {}, {}", patt, row, col);
        }
        if row / 100 >= col {
            row
        } else {
            col
        }
    });
    lines.sum()
}

fn q2(input: &str) -> usize {
    let lines = input.split("\n\n").map(|pat| {
        let patt = pat.lines().collect_vec();
        let row = get_row(&patt, 2);
        let col = get_col(&patt, 2);
        if row == 0 && col == 0 {
            println!("{:?} => {}, {}", patt, row, col);
        }
        if row / 100 >= col {
            row
        } else {
            col
        }
    });
    lines.sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";

        let res = q1(input);
        // println!("Test: {} => {:?}", lava.pattern, lava.condition);
        assert_eq!(5, res);
    }

    #[test]
    fn test1_2() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";

        let res = q2(input);
        // println!("Test: {} => {:?}", lava.pattern, lava.condition);
        assert_eq!(300, res);
    }

    #[test]
    fn test2() {
        let input = "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        let res = q1(input);
        assert_eq!(400, res);
    }

    #[test]
    fn test2_2() {
        let input = "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        let res = q2(input);
        assert_eq!(100, res);
    }

    #[test]
    fn test3() {
        let input = ".###...##
##..#....
##..#..#.
......###
#..##..##
######.#.
.#.###..#
#####.#.#
##..#..##
##..#..##
#####.#.#
.#.###..#
######.#.
#..##..##
......###
##..#..#.
##..#....";

        let res = q1(input);
        assert_eq!(900, res);
    }
}
