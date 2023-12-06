pub fn sln(input: String) {
    println!("{}", q1(&input));
    println!("{}", q2(&input));
}

#[derive(Debug)]
struct Race {
    time: u64,
    dist: u64,
}

impl Race {
    fn new(time: u64, dist: u64) -> Self {
        Self { time, dist }
    }

    fn acc(&self, n: u64) -> bool {
        (self.time - n) * n > self.dist
    }
}

fn eval(times: Vec<u64>, dists: Vec<u64>) -> u64 {
    let races = gen_races(times, dists);
    races
        .iter()
        .map(|r| -> u64 {
            for i in 1..r.time / 2 {
                if r.acc(i) {
                    return r.time - i * 2 + 1;
                }
            }
            0
        })
        .fold(1u64, |a, v| a * v)
}

fn parse_str(input: &String, n: usize, f: &dyn Fn(&str) -> String) -> Vec<u64> {
    let intermed = input
        .lines()
        .nth(n)
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim();
    let sp = f(intermed);
        sp.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn unjoined(input: &str) -> String {
    String::from(input)
}
fn joined(input: &str) -> String {
    input.replace(" ", "")
}

fn gen_races(times: Vec<u64>, dists: Vec<u64>) -> Vec<Race> {
    times
        .into_iter()
        .zip(dists)
        .map(|(t, d)| Race::new(t, d))
        .collect()
}

fn q1(input: &String) -> u64 {
    let times = parse_str(input, 0, &unjoined);
    let dists = parse_str(input, 1, &unjoined);
    eval(times, dists)
}

fn q2(input: &String) -> u64 {
    let times = parse_str(input, 0, &joined);
    let dists = parse_str(input, 1, &joined);
    eval(times, dists)
}
