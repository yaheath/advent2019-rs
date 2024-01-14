use std::vec::Vec;
use ya_advent_lib::read::read_input;

fn fuel_for(mass: i64) -> i64 {
    (mass / 3 - 2).max(0)
}

fn part1(input: &[i64]) -> i64 {
    input.iter().map(|v| fuel_for(*v)).sum()
}

fn total_fuel_for(mass: i64) -> i64 {
    let mut sum = 0;
    let mut last = mass;
    while last > 0 {
        let f = fuel_for(last);
        sum += f;
        last = f;
    }
    sum
}

fn part2(input: &[i64]) -> i64 {
    input.iter().map(|v| total_fuel_for(*v)).sum()
}

fn main() {
    let input: Vec<i64> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day01_test() {
        assert_eq!(fuel_for(12), 2);
        assert_eq!(total_fuel_for(12), 2);
        assert_eq!(fuel_for(14), 2);
        assert_eq!(total_fuel_for(14), 2);
        assert_eq!(fuel_for(1969), 654);
        assert_eq!(total_fuel_for(1969), 966);
        assert_eq!(fuel_for(100756), 33583);
        assert_eq!(total_fuel_for(100756), 50346);
    }
}
