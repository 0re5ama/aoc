pub fn sln(input: String) {
    println!("{}", q1(&input));
    println!("{}", q2(&input));
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

fn q1(_input: &String) -> u64 {
    0
}

fn q2(_input: &String) -> u64 {
    0
}
