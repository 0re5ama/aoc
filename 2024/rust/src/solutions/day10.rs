pub fn silver(input: &String) -> usize {
    if let Some((rules, updates)) = input.split_once("\n\n") {
        let rules: Vec<(usize, usize)> = rules
            .lines()
            .filter_map(|l| {
                if let Some((left, right)) = l.split_once("|") {
                    let left = left.parse::<usize>().unwrap_or(0);
                    let right = right.parse::<usize>().unwrap_or(0);
                    return Some((left, right));
                }
                None
            })
            .collect();

        let mut valid_updates: Vec<&str> = vec![];

        let updates = updates
            .lines()
            .map(|l| for (x, i) in l.split(",").enumerate() {});
    }
    0
}

pub fn gold(_input: &String) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn silver_test() {
        let input = String::from(
            "
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
",
        );
        assert_eq!(silver(&input), 143);
    }

    #[test]
    fn gold_test() {
        let input = String::from("");
        assert_eq!(gold(&input), 0);
    }
}
