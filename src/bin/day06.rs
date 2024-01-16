use std::collections::HashMap;
use std::str::FromStr;
use std::vec::Vec;
use ya_advent_lib::read::read_input;

struct Input {
    cent: String,
    sat: String,
}

impl FromStr for Input {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cent, sat) = s.split_once(')').unwrap();
        Ok(Input{cent: cent.to_owned(), sat: sat.to_owned()})
    }
}

fn setup(input: &[Input]) -> HashMap<&str, &str> {
    input.iter().map(|i| (i.sat.as_str(), i.cent.as_str())).collect()
}

fn part1(input: &[Input]) -> usize {
    let map = setup(input);
    let mut n = 0;
    for k in map.keys() {
        let mut obj = *k;
        while obj != "COM" {
            n += 1;
            obj = map[obj];
        }
    }
    n
}

fn part2(input: &[Input]) -> usize {
    let map = setup(input);
    let mut santa_path: HashMap<&str, usize> = HashMap::new();
    let mut obj = map["SAN"];
    let mut steps = 0;
    while obj != "COM" {
        santa_path.insert(obj, steps);
        steps += 1;
        obj = map[obj];
    }
    santa_path.insert("COM", steps);
    obj = map["YOU"];
    steps = 0;
    while obj != "COM" {
        if santa_path.contains_key(obj) {
            return santa_path[obj] + steps;
        }
        steps += 1;
        obj = map[obj];
    }
    steps + santa_path["COM"]
}

fn main() {
    let input: Vec<Input> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day06_test() {
        let input: Vec<Input> = test_input(include_str!("day06.testinput"));
        assert_eq!(part1(&input), 42);
        let input: Vec<Input> = test_input(include_str!("day06.testinput2"));
        assert_eq!(part2(&input), 4);
    }
}
