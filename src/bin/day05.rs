use std::vec::Vec;
use ya_advent_lib::read::read_input;
extern crate advent2019;
use advent2019::intcode::{IntcodeVM, ProgMem};

fn part1(input: &ProgMem) -> i64 {
    let mut last_output = 0;
    let mut vm = IntcodeVM::with_mem(input);
    let mut output = |v| {
        last_output = v;
    };
    vm.run_with_cb(&mut || Some(1), &mut output).unwrap();
    last_output
}

fn part2(input: &ProgMem) -> i64 {
    let mut last_output = 0;
    let mut vm = IntcodeVM::with_mem(input);
    let mut output = |v| {
        last_output = v;
    };
    vm.run_with_cb(&mut || Some(5), &mut output).unwrap();
    last_output
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
    fn day05_test() {
        let input: Vec<ProgMem> = test_input("1002,4,3,4,33");
        let mut vm = IntcodeVM::with_mem(&input[0]);
        let result: Vec<ProgMem> = test_input("1002,4,3,4,99");
        vm.run().unwrap();
        assert_eq!(vm.mem, result[0].0);

        let input: Vec<ProgMem> = test_input("3,9,8,9,10,9,4,9,99,-1,8");
        let mut vm = IntcodeVM::with_mem(&input[0]);
        vm.run_with_cb(&mut || Some(8), &mut |v| assert_eq!(v, 1))
            .unwrap();
        let mut vm = IntcodeVM::with_mem(&input[0]);
        vm.run_with_cb(&mut || Some(2), &mut |v| assert_eq!(v, 0))
            .unwrap();

        let input: Vec<ProgMem> = test_input("3,9,7,9,10,9,4,9,99,-1,8");
        let mut vm = IntcodeVM::with_mem(&input[0]);
        vm.run_with_cb(&mut || Some(8), &mut |v| assert_eq!(v, 0))
            .unwrap();
        let mut vm = IntcodeVM::with_mem(&input[0]);
        vm.run_with_cb(&mut || Some(2), &mut |v| assert_eq!(v, 1))
            .unwrap();

        let input: Vec<ProgMem> = test_input("3,3,1108,-1,8,3,4,3,99");
        let mut vm = IntcodeVM::with_mem(&input[0]);
        vm.run_with_cb(&mut || Some(8), &mut |v| assert_eq!(v, 1))
            .unwrap();
        let mut vm = IntcodeVM::with_mem(&input[0]);
        vm.run_with_cb(&mut || Some(2), &mut |v| assert_eq!(v, 0))
            .unwrap();

        let input: Vec<ProgMem> = test_input("3,3,1107,-1,8,3,4,3,99");
        let mut vm = IntcodeVM::with_mem(&input[0]);
        vm.run_with_cb(&mut || Some(8), &mut |v| assert_eq!(v, 0))
            .unwrap();
        let mut vm = IntcodeVM::with_mem(&input[0]);
        vm.run_with_cb(&mut || Some(2), &mut |v| assert_eq!(v, 1))
            .unwrap();

        let input: Vec<ProgMem> = test_input("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9");
        let mut vm = IntcodeVM::with_mem(&input[0]);
        vm.run_with_cb(&mut || Some(0), &mut |v| assert_eq!(v, 0))
            .unwrap();
        let mut vm = IntcodeVM::with_mem(&input[0]);
        vm.run_with_cb(&mut || Some(2), &mut |v| assert_eq!(v, 1))
            .unwrap();

        let input: Vec<ProgMem> = test_input("3,3,1105,-1,9,1101,0,0,12,4,12,99,1");
        let mut vm = IntcodeVM::with_mem(&input[0]);
        vm.run_with_cb(&mut || Some(0), &mut |v| assert_eq!(v, 0))
            .unwrap();
        let mut vm = IntcodeVM::with_mem(&input[0]);
        vm.run_with_cb(&mut || Some(2), &mut |v| assert_eq!(v, 1))
            .unwrap();

        let input: Vec<ProgMem> = test_input("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99");
        let mut vm = IntcodeVM::with_mem(&input[0]);
        vm.run_with_cb(&mut || Some(-1), &mut |v| assert_eq!(v, 999))
            .unwrap();
        let mut vm = IntcodeVM::with_mem(&input[0]);
        vm.run_with_cb(&mut || Some(8), &mut |v| assert_eq!(v, 1000))
            .unwrap();
        let mut vm = IntcodeVM::with_mem(&input[0]);
        vm.run_with_cb(&mut || Some(80), &mut |v| assert_eq!(v, 1001))
            .unwrap();
    }
}
