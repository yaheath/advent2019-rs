use std::vec::Vec;
use itertools::Itertools;
use ya_advent_lib::read::read_input;
extern crate advent2019;
use advent2019::intcode::{IntcodeVM, ProgMem};

fn part1(input: &ProgMem) -> i64 {
    let mut vm = IntcodeVM::with_mem(&input);
    vm.mem[1] = 12;
    vm.mem[2] = 2;
    vm.run().unwrap();
    vm.mem[0]
}

fn part2(input: &ProgMem) -> i64 {
    (0..=99)
        .cartesian_product(0..=99)
        .map(|(n, v)| {
            let mut vm = IntcodeVM::with_mem(&input);
            vm.mem[1] = n;
            vm.mem[2] = v;
            vm.run().unwrap();
            (n, v, vm.mem[0])
        })
        .find(|(_, _, res)| *res == 19690720)
        .map(|(n, v, _)| n * 100 + v)
        .unwrap()
}

fn main() {
    let input: Vec<ProgMem> = read_input();
    println!("Part 1: {}", part1(&input[0]));
    println!("Part 2: {}", part2(&input[0]));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day02_test() {
        let input: Vec<ProgMem> = test_input("1,9,10,3,2,3,11,0,99,30,40,50");
        let mut vm = IntcodeVM::with_mem(&input[0]);
        let result: Vec<ProgMem> = test_input("3500,9,10,70,2,3,11,0,99,30,40,50");
        vm.run().unwrap();
        assert_eq!(vm.mem, result[0].0);

        let input: Vec<ProgMem> = test_input("1,1,1,4,99,5,6,0,99");
        let mut vm = IntcodeVM::with_mem(&input[0]);
        let result: Vec<ProgMem> = test_input("30,1,1,4,2,5,6,0,99");
        vm.run().unwrap();
        assert_eq!(vm.mem, result[0].0);
    }
}
