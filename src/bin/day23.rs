use std::collections::VecDeque;
use std::vec::Vec;
use ya_advent_lib::read::read_input;
extern crate advent2019;
use advent2019::intcode::{IntcodeVM, ProgMem, RunErr};

fn run(input: &ProgMem, part2: bool) -> i64 {
    let mut nodes: Vec<IntcodeVM> = Vec::new();
    let mut input_queue: Vec<VecDeque<i64>> = Vec::new();
    let mut output_queue: Vec<VecDeque<i64>> = Vec::new();
    let mut nat_buffer: Option<(i64, i64)> = None;
    let mut last_nat_y: Option<i64> = None;

    for n in 0..50 {
        let mut vm = IntcodeVM::with_mem(input);
        vm.input_queue.push_back(n);
        nodes.push(vm);
        input_queue.push(VecDeque::new());
        output_queue.push(VecDeque::new());
    }
    loop {
        for (idx, node) in nodes.iter_mut().enumerate() {
            if input_queue[idx].is_empty() {
                node.input_queue.push_back(-1);
            } else {
                let Some(q) = input_queue.get_mut(idx) else {
                    panic!();
                };
                while let Some(v) = q.pop_front() {
                    node.input_queue.push_back(v);
                }
            }
            match node.run_with_cb(&mut || None, &mut |val| {
                if let Some(q) = output_queue.get_mut(idx) {
                    q.push_back(val)
                }
            }) {
                Ok(_) => {
                    panic!("program exited");
                }
                Err(RunErr::InputNeeded) => {}
                Err(_) => {
                    panic!("program error");
                }
            }
            if let Some(vec) = output_queue.get_mut(idx) {
                while vec.len() >= 3 {
                    if vec[0] == 255 {
                        if !part2 {
                            return vec[2];
                        }
                        nat_buffer = Some((vec[1], vec[2]));
                    } else {
                        let Some(q) = input_queue.get_mut(vec[0] as usize) else {
                            panic!();
                        };
                        q.push_back(vec[1]);
                        q.push_back(vec[2]);
                    }
                    vec.pop_front();
                    vec.pop_front();
                    vec.pop_front();
                }
            }
        }
        if part2 && input_queue.iter().all(|q| q.is_empty()) {
            assert!(nat_buffer.is_some());
            let nbx = nat_buffer.as_ref().map(|t| t.0).unwrap();
            let nby = nat_buffer.as_ref().map(|t| t.1).unwrap();
            if let Some(last_y) = last_nat_y {
                if last_y == nby {
                    return last_y;
                }
            }
            nodes.get_mut(0).map(|node| {
                node.input_queue.push_back(nbx);
                node.input_queue.push_back(nby);
            });
            last_nat_y = Some(nby);
        }
    }
}

fn part1(input: &ProgMem) -> i64 {
    run(input, false)
}

fn part2(input: &ProgMem) -> i64 {
    run(input, true)
}

fn main() {
    let input: Vec<ProgMem> = read_input();
    let p1 = part1(&input[0]);
    println!("Part 1: {p1}");
    let p2 = part2(&input[0]);
    println!("Part 2: {p2}");
}
