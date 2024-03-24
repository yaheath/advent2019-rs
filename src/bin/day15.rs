use std::collections::{HashMap, VecDeque};
use std::vec::Vec;
use ya_advent_lib::coords::{CDir, Coord2D};
use ya_advent_lib::infinite_grid::InfiniteGrid;
use ya_advent_lib::read::read_input;
extern crate advent2019;
use advent2019::intcode::{IntcodeVM, ProgMem, RunErr};

enum MoveResult {
    Moved,
    HitWall,
    FoundGoal,
}

struct Droid {
    vm: IntcodeVM,
}
impl Droid {
    fn with_mem(mem: &ProgMem) -> Self {
        let vm = IntcodeVM::with_mem(&mem);
        Self { vm }
    }
    fn step(&mut self, dir: CDir) -> MoveResult {
        let v = match dir {
            CDir::N => 1,
            CDir::S => 2,
            CDir::W => 3,
            CDir::E => 4,
        };
        self.vm.input_queue.push_back(v);
        let mut result = -1;
        match self.vm.run_with_cb(&mut || None, &mut |v| result = v) {
            Ok(_) => { panic!("program exited"); },
            Err(RunErr::InputNeeded) => {},
            Err(_) => { panic!("program error"); },
        }
        match result {
            0 => MoveResult::HitWall,
            1 => MoveResult::Moved,
            2 => MoveResult::FoundGoal,
            _ => panic!("unexpected result {result}"),
        }
    }
}

#[derive(Copy, Clone)]
enum Cell {
    Unexplored,
    Wall,
    Open(usize),
    Goal(usize),
}

fn recurse(steps: usize, loc: Coord2D, droid: &mut Droid, grid: &mut InfiniteGrid<Cell>) {
    for dir in [CDir::N, CDir::E, CDir::W, CDir::S] {
        let neigh = grid.get_c(loc + dir);
        match neigh {
            Cell::Wall => { continue; },
            Cell::Open(n) if n <= steps + 1 => { continue; },
            Cell::Goal(n) if n <= steps + 1 => { continue; },
            _ => {},
        }
        match droid.step(dir) {
            MoveResult::HitWall => {
                grid.set_c(loc + dir, Cell::Wall);
                continue;
            },
            MoveResult::Moved => {
                grid.set_c(loc + dir, Cell::Open(steps + 1));
            },
            MoveResult::FoundGoal => {
                grid.set_c(loc + dir, Cell::Goal(steps + 1));
            }
        }
        recurse(steps + 1, loc + dir, droid, grid);
        droid.step(-dir);
    }
}

fn explore(droid: &mut Droid) -> InfiniteGrid<Cell> {
    let mut grid = InfiniteGrid::new(Cell::Unexplored);
    let start = Coord2D::new(0, 0);
    grid.set_c(start, Cell::Open(0));
    recurse(0, start, droid, &mut grid);
    grid
}

fn part1(input: &ProgMem) -> (usize, InfiniteGrid<Cell>) {
    let mut droid = Droid::with_mem(input);
    let grid = explore(&mut droid);
    let steps = grid.iter().find_map(|(_, cell)| match cell {
        Cell::Goal(n) => Some(*n),
        _ => None,
    })
    .unwrap();
    (steps, grid)
}

fn part2(grid: &InfiniteGrid<Cell>) -> usize {
    let start = grid.iter().find_map(|((x, y), c)| match c {
        Cell::Goal(_) => Some(Coord2D::new(*x, *y)),
        _ => None,
    }).unwrap();
    let mut traversed: HashMap<Coord2D, usize> = HashMap::new();
    let mut queue: VecDeque<(Coord2D, usize)> = VecDeque::new();
    queue.push_back((start, 0));
    traversed.insert(start, 0);
    while let Some((pos, steps)) = queue.pop_front() {
        pos.neighbors4().iter()
            .filter(|n| matches!(grid.get_c(**n), Cell::Open(_)))
            .for_each(|n| {
                if !traversed.contains_key(n) || traversed[n] > steps + 1 {
                    queue.push_back((*n, steps + 1));
                    traversed.insert(*n, steps + 1);
                }
            });
    }
    *traversed.values().max().unwrap()
}

fn main() {
    let input: Vec<ProgMem> = read_input();
    let (part1, grid) = part1(&input[0]);
    println!("Part 1: {part1}");
    println!("Part 2: {}", part2(&grid));
}
#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day15_test() {
        let input = test_input::<String>(
" ##   
#..## 
#.#..#
#.O.# 
 ###  
");
        let grid = InfiniteGrid::from_input(&input, Cell::Unexplored,
            |c, _, _| match c{
                '.' => Some(Cell::Open(0)),
                '#' => Some(Cell::Wall),
                'O' => Some(Cell::Goal(0)),
                _ => None,
            }
        );
        assert_eq!(part2(&grid), 4);
    }
}
