use std::vec::Vec;
use ya_advent_lib::read::read_input;
extern crate advent2019;
use advent2019::intcode::{IntcodeVM, ProgMem};

fn part1(input: &ProgMem) -> i64 {
    let mut vm = IntcodeVM::with_mem(&input);

// if any of A B or C are 0 and D is 1, then jump
"NOT A T
NOT B J
OR T J
NOT C T
OR T J
AND D J
WALK
".chars().map(|c| c as u8 as i64).for_each(|c| vm.input_queue.push_back(c));

    let mut out = 0;
    //vm.run_interactive(&mut |v| {out = v;}).unwrap();
    vm.run_with_cb(&mut || None, &mut |v| {out = v;}).unwrap();
    out
}

fn part2(input: &ProgMem) -> i64 {
    let mut vm = IntcodeVM::with_mem(&input);
"OR B J
AND C J
NOT J J
AND D J
AND H J
NOT A T
OR T J
RUN
".chars().map(|c| c as u8 as i64).for_each(|c| vm.input_queue.push_back(c));

    let mut out = 0;
    vm.run_with_cb(&mut || None, &mut |v| {out = v;}).unwrap();
    out
}

fn main() {
    let input: Vec<ProgMem> = read_input();
    let p1 = part1(&input[0]);
    println!("Part 1: {p1}");
    let p2 = part2(&input[0]);
    println!("Part 2: {p2}");
}
