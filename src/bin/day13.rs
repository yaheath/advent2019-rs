use std::vec::Vec;
use ya_advent_lib::read::read_input;
extern crate advent2019;
use advent2019::intcode::{IntcodeVM, ProgMem, RunErr};

fn part1(input: &ProgMem) -> usize {
    let mut vm = IntcodeVM::with_mem(input);
    let mut outp = Vec::new();
    vm.run_with_cb(&mut || None, &mut |v| outp.push(v)).unwrap();
    outp.chunks(3).filter(|v| v[2] == 2).count()
}

fn part2(input: &ProgMem) -> i64 {
    let mut vm = IntcodeVM::with_mem(input);
    let mut score = 0;
    let mut paddle_pos = 0;
    let mut ball_pos = 0;
    vm.mem[0] = 2;
    let mut done = false;
    let mut outp = Vec::new();
    while !done {
        outp.clear();
        match vm.run_with_cb(&mut || None, &mut |v| outp.push(v)) {
            Ok(_) => {
                done = true;
            }
            Err(RunErr::InputNeeded) => {}
            Err(_) => {
                panic!();
            }
        }
        outp.chunks(3).for_each(|v| {
            if v[0] == -1 && v[1] == 0 {
                score = v[2];
            } else {
                match v[2] {
                    3 => {
                        paddle_pos = v[0];
                    }
                    4 => {
                        ball_pos = v[0];
                    }
                    _ => {}
                };
            }
        });
        vm.input_queue.push_back(if paddle_pos < ball_pos {
            1
        } else if paddle_pos > ball_pos {
            -1
        } else {
            0
        });
    }
    score
}

fn main() {
    let input: Vec<ProgMem> = read_input();
    println!("Part 1: {}", part1(&input[0]));
    println!("Part 2: {}", part2(&input[0]));
}
