pub fn sln(input: String, q: Option<u8>) {
    match q {
        Some(1) => println!("{}", q1(&input)),
        Some(2) => println!("{}", q2(&input)),
        _ => {
            println!("{}", q1(&input));
            println!("{}", q2(&input));
        }
    }
}

struct _Number<'a> {
    ix: usize,
    num: &'a str,
}

/*
impl<'a> _Number<'a> {
    fn new(ix: usize, num: &'a str) -> Self { Self { ix, num } }
}
*/

fn q1(_input: &str) -> u64 {
    0
}

fn q2(_input: &str) -> u64 {
    0
}
