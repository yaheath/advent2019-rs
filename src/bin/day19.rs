use std::vec::Vec;
use itertools::Itertools;
use ya_advent_lib::read::read_input;
extern crate advent2019;
use advent2019::intcode::{IntcodeVM, ProgMem};

fn test_loc(input: &ProgMem, x: i64, y: i64) -> i64 {
    let mut vm = IntcodeVM::with_mem(&input);
    vm.input_queue.push_back(x);
    vm.input_queue.push_back(y);
    let mut out = -1;
    vm.run_with_cb(&mut || None, &mut |v| out = v).unwrap();
    assert_ne!(out, -1);
    out
}

fn part1(input: &ProgMem) -> usize {
    (0..50).cartesian_product(0..50)
        .filter(|(x, y)| test_loc(input, *x, *y) != 0)
        .count()
}

fn part2(input: &ProgMem) -> i64 {
    // x1,y1 is the top-right corner of the square
    // search down to find the upper edge of the beam
    let mut x1 = 100;
    let mut y1 = 0;
    while test_loc(input, x1, y1) == 1 {
        y1 -= 1;
    }
    y1 += 1;

    // Now move outward along the upper edge of the beam
    // until the opposite corner is in the beam.
    // The opposite corner is (x1-99, y1+99)
    loop {
        if test_loc(input, x1 + 1, y1) == 1 {
            x1 += 1;
        }
        else {
            y1 += 1;
        }
        if test_loc(input, x1 - 99, y1 + 99) == 1 {
            return (x1 - 99) * 10000 + y1;
        }
    }
}

fn main() {
    let input: Vec<ProgMem> = read_input();
    println!("Part 1: {}", part1(&input[0]));
    println!("Part 2: {}", part2(&input[0]));
}
