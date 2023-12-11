pub fn sln(input: String, q: Option<u8>) {
    let _ = input;
    match q {
        Some(1) => println!("{}", q1()),
        Some(2) => println!("{}", q2()),
        _ => {
            println!("{}", q1());
            println!("{}", q2());
        }
    }
}

fn q1() -> u64 {
    0
}

fn q2() -> u64 {
    0
}
