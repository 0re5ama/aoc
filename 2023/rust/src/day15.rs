use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

pub fn sln(input: String, q: Option<u8>) {
    let input = input.trim();
    match q {
        Some(1) => println!("{}", q1(input)),
        Some(2) => println!("{}", q2(input)),
        _ => {
            println!("{}", q1(input));
            println!("{}", q2(input));
        }
    }
}

#[derive(Debug)]
enum Operation {
    Remove,
    Add,
}

#[derive(Debug)]
struct Step {
    label: String,
    op: Operation,
    lens: usize,
}

impl Step {
    fn new(label: &str, op: &str, lens: &str) -> Self {
        Self {
            label: label.to_string(),
            op: match op {
                "-" => Operation::Remove,
                "=" => Operation::Add,
                _ => panic!("muda muda"),
            },
            lens: lens.parse().unwrap_or(0),
        }
    }
}

fn hash(pat: &str) -> usize {
    let mut val = 0;
    pat.chars().for_each(|c| {
        val += c as usize;
        val *= 17;
        val %= 256;
    });
    val
}

fn q1(input: &str) -> usize {
    input.split(',').map(hash).sum()
}

fn operate(map: &HashMap<usize, Vec<(String, usize)>>) -> usize {
    let keys = map.keys().clone();
    let keys = keys.sorted_by(|a, b| a.cmp(b));

    keys.map(|k| {
        map.get(k)
            .unwrap()
            .iter()
            .enumerate()
            .map(|(i, (_label, lens))| {
                #[cfg(test)]
                println!(
                    "{}: {} (box {}) * {} * {} = {}",
                    _label,
                    k + 1,
                    k,
                    i + 1,
                    lens,
                    (k + 1) * (i + 1) * lens
                );
                (k + 1) * (i + 1) * lens
            })
            .sum::<usize>()
    })
    .sum()
}

fn q2(input: &str) -> usize {
    let mut boxes: HashMap<usize, Vec<(String, usize)>> = HashMap::new();
    let re = Regex::new(r"(?<label>\w+)(?<op>[=-])(?<lens>\d*)").unwrap();
    input
        .split(',')
        .map(|pat| {
            if let Some(caps) = re.captures(pat) {
                let label = &caps["label"];
                let op = &caps["op"];
                let lens = &caps["lens"];
                Step::new(label, op, lens)
            } else {
                Step::new("test", "=", "0")
            }
        })
        .for_each(|step| {
            let hash = hash(step.label.as_str());
            match step.op {
                Operation::Remove => {
                    if let Some(vec) = boxes.get_mut(&hash) {
                        if let Some(pos) =
                            vec.iter().position(|(label, _lens)| *label == step.label)
                        {
                            vec.remove(pos);
                        }
                    }
                }
                Operation::Add => {
                    if let Some(vec) = boxes.get_mut(&hash) {
                        if let Some(dd) = vec.iter_mut().find(|(label, _lens)| *label == step.label)
                        {
                            dd.1 = step.lens;
                        } else {
                            vec.push((step.label, step.lens));
                        }
                    } else {
                        let labels = vec![(step.label, step.lens)];
                        boxes.insert(hash, labels);
                    }
                }
            }
        });
    operate(&boxes)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let input = "rn=1";
        let res = q1(input);
        assert_eq!(30, res);
    }

    #[test]
    fn test2() {
        let input = "cm-";
        let res = q1(input);
        assert_eq!(253, res);
    }

    #[test]
    fn test3() {
        let input = "qp=3";
        let res = q1(input);
        assert_eq!(97, res);
    }

    #[test]
    fn test4() {
        let input = "cm=2";
        let res = q1(input);
        assert_eq!(47, res);
    }

    #[test]
    fn test5() {
        let input = "qp-";
        let res = q1(input);
        assert_eq!(14, res);
    }

    #[test]
    fn test6() {
        let input = "pc=4";
        let res = q1(input);
        assert_eq!(180, res);
    }

    #[test]
    fn test7() {
        let input = "ot=9";
        let res = q1(input);
        assert_eq!(9, res);
    }

    #[test]
    fn test8() {
        let input = "ab=5";
        let res = q1(input);
        assert_eq!(197, res);
    }

    #[test]
    fn test9() {
        let input = "pc-";
        let res = q1(input);
        assert_eq!(48, res);
    }

    #[test]
    fn test10() {
        let input = "pc=6";
        let res = q1(input);
        assert_eq!(214, res);
    }

    #[test]
    fn test11() {
        let input = "ot=7";
        let res = q1(input);
        assert_eq!(231, res);
    }

    #[test]
    fn test12() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let res = q1(input);
        assert_eq!(1320, res);
    }

    #[test]
    fn test13() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let res = q2(input);
        assert_eq!(145, res);
    }
}
