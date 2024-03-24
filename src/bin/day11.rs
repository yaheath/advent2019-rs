use std::vec::Vec;
use ya_advent_lib::coords::{CDir, Coord2D, Turn};
use ya_advent_lib::infinite_grid::InfiniteGrid;
use ya_advent_lib::read::read_input;
extern crate advent2019;
use advent2019::intcode::{IntcodeVM, ProgMem, RunErr};

fn run_robot(prog: &ProgMem, initial: bool) -> InfiniteGrid<bool> {
    let mut pos = Coord2D::new(0, 0);
    let mut dir = CDir::N;
    let mut grid = InfiniteGrid::new(false);
    let mut vm = IntcodeVM::with_mem(prog);
    grid.set_c(pos, initial);
    loop {
        vm.input_queue
            .push_back(if grid.get_c(pos) { 1 } else { 0 });
        let mut outp = Vec::new();
        match vm.run_with_cb(&mut || None, &mut |v| {
            outp.push(v);
        }) {
            Ok(_) => {
                break;
            }
            Err(RunErr::InputNeeded) => {}
            Err(_) => {
                panic!();
            }
        }
        assert_eq!(outp.len(), 2);
        grid.set_c(pos, outp[0] == 1);
        dir += if outp[1] == 0 { Turn::L } else { Turn::R };
        pos += dir;
    }
    grid
}

fn part1(input: &ProgMem) -> usize {
    run_robot(input, false).iter().count()
}

fn part2(input: &ProgMem) {
    let grid = run_robot(input, true);
    grid.print(|v| if v { '\u{2588}' } else { ' ' });
}

fn main() {
    let input: Vec<ProgMem> = read_input();
    println!("Part 1: {}", part1(&input[0]));
    println!("Part 2:");
    part2(&input[0]);
}
