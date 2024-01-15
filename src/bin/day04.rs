use std::vec::Vec;
use itertools::Itertools;
use ya_advent_lib::range::range_from_str;
use ya_advent_lib::read::read_input;

fn valid_password(pw: usize) -> bool {
    let pws = format!("{pw:06}");
    pws.chars().tuple_windows().all(|(a,b)| a <= b)
        && pws.chars().tuple_windows().any(|(a,b)| a == b)
}

fn valid_password_p2(pw: usize) -> bool {
    let pws = format!("{pw:06}");
    pws.chars().tuple_windows().all(|(a,b)| a <= b)
        && pws.chars().dedup_with_count().any(|(c,_)| c == 2)
}

fn part1(input: &str) -> usize {
    let range = range_from_str(input, true).unwrap();
    range.filter(|v| valid_password(*v)).count()
}

fn part2(input: &str) -> usize {
    let range = range_from_str(input, true).unwrap();
    range.filter(|v| valid_password_p2(*v)).count()
}

fn main() {
    let input: Vec<String> = read_input();
    println!("Part 1: {}", part1(&input[0]));
    println!("Part 2: {}", part2(&input[0]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day04_test() {
        assert!(valid_password(111111));
        assert!(!valid_password_p2(111111));
        assert!(!valid_password(223450));
        assert!(!valid_password_p2(223450));
        assert!(!valid_password(123789));
        assert!(!valid_password_p2(123789));
        assert!(valid_password(112233));
        assert!(valid_password_p2(112233));
        assert!(valid_password(123444));
        assert!(!valid_password_p2(123444));
        assert!(valid_password(111122));
        assert!(valid_password_p2(111122));
    }
}
