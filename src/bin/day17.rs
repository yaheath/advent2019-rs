use std::vec::Vec;
use ya_advent_lib::coords::{CDir, Coord2D};
use ya_advent_lib::grid::Grid;
use ya_advent_lib::read::read_input;
extern crate advent2019;
use advent2019::intcode::{IntcodeVM, ProgMem, RunErr};

#[derive(Copy, Clone, Eq, PartialEq)]
enum Cell {
    Empty,
    Scaffold,
    Robot(CDir),
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '.' => Cell::Empty,
            '#' => Cell::Scaffold,
            '^' => Cell::Robot(CDir::N),
            'v' => Cell::Robot(CDir::S),
            '<' => Cell::Robot(CDir::W),
            '>' => Cell::Robot(CDir::E),
            _ => panic!(),
        }
    }
}

fn part1(input: &ProgMem) -> i64 {
    let mut vm = IntcodeVM::with_mem(&input);
    let mut outp = String::new();
    vm.run_with_cb(&mut || None, &mut |v| outp.push(v as u8 as char)).unwrap();
    //println!("{outp}");
    let lines: Vec<String> = outp.lines().map(|l| l.into()).collect();
    let grid: Grid<Cell> = Grid::from_input(&lines, Cell::Empty, 0);
    grid.iter_with_coord()
        .filter_map(|(c, x, y)| {
            if c == Cell::Scaffold &&
               Coord2D::new(x, y).neighbors4().iter()
               .all(|n| grid.get_or_default(n.x, n.y, Cell::Empty) == Cell::Scaffold)
            {
                Some(x * y)
            }
            else { None }
        })
        .sum()
}

fn part2(input: &ProgMem) -> i64 {
    0
}

fn main() {
    let input: Vec<ProgMem> = read_input();
    println!("Part 1: {}", part1(&input[0]));
    println!("Part 2: {}", part2(&input[0]));
}
