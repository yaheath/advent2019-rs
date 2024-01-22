use std::collections::HashMap;
use std::vec::Vec;
use ya_advent_lib::algorithm::dijkstra;
use ya_advent_lib::coords::{Coord2D, CDir};
use ya_advent_lib::grid::Grid;
use ya_advent_lib::read::read_input;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Cell {
    Null,
    Wall,
    Open,
    LabelPart(char),
    InnerPortal([char;2]),
    OuterPortal([char;2]),
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '.' => Cell::Open,
            'A' ..= 'Z' => Cell::LabelPart(c),
            '#' => Cell::Wall,
            _ => Cell::Null,
        }
    }
}
impl From<Cell> for char {
    fn from(c: Cell) -> Self {
        match c {
            Cell::Null => ' ',
            Cell::Wall => '#',
            Cell::Open => '.',
            Cell::InnerPortal(a) => a[0],
            Cell::OuterPortal(a) => a[0],
            Cell::LabelPart(c) => c,
        }
    }
}

struct Map {
    grid: Grid<Cell>,
    inners: HashMap<[char;2], Coord2D>,
    outers: HashMap<[char;2], Coord2D>,
}

fn mkmap(input: &[String]) -> Map {
    let mut grid = Grid::from_input(input, Cell::Null, 1);
    let mut inners = HashMap::new();
    let mut outers = HashMap::new();
    let grid_w = grid.x_bounds().end - grid.x_bounds().start;
    let grid_h = grid.y_bounds().end - grid.y_bounds().start;

    for y in grid.y_bounds() {
        for x in grid.x_bounds() {
            match grid.get(x, y) {
                Cell::LabelPart(c1) => {
                    let coord1 = Coord2D::new(x, y);
                    let (coord2, c2) = coord1.neighbors4().iter()
                        .find_map(|n| match grid.get_c(*n) {
                            Cell::LabelPart(c2) => Some((*n, c2)),
                            _ => None,
                        })
                        .unwrap();
                    let portalcoord = match (coord2 - coord1).into() {
                        (1, 0) => {
                            if matches!(grid.get_c(coord1 + CDir::W), Cell::Open) {
                                coord1 + CDir::W
                            }
                            else {
                                coord2 + CDir::E
                            }
                        },
                        (0, 1) => {
                            if matches!(grid.get_c(coord1 + CDir::N), Cell::Open) {
                                coord1 + CDir::N
                            }
                            else {
                                coord2 + CDir::S
                            }
                        },
                        _ => panic!(),
                    };
                    grid.set_c(coord1, Cell::Null);
                    grid.set_c(coord2, Cell::Null);
                    let portal = if portalcoord.x < 4 || portalcoord.y < 4 ||
                        portalcoord.x > grid_w - 6 || portalcoord.y > grid_h - 6 {
                            outers.insert([c1, c2], portalcoord);
                            Cell::OuterPortal([c1, c2])
                    }
                    else {
                            inners.insert([c1, c2], portalcoord);
                            Cell::InnerPortal([c1, c2])
                    };
                    grid.set_c(portalcoord, portal);
                },
                _ => {},
            }
        }
    }
    // grid.print();
    Map{grid, inners, outers}
}

fn traverse(input: &[String], part2: bool) -> usize {
    let map = mkmap(input);
    let start_loc:Coord2D =
        map.grid.find(|c, _, _| matches!(c, Cell::OuterPortal(['A','A'])))
        .unwrap()
        .into();
    let target_loc:Coord2D =
        map.grid.find(|c, _, _| matches!(c, Cell::OuterPortal(['Z','Z'])))
        .unwrap()
        .into();
    let start = (start_loc, 0);
    let target = (target_loc, 0);

    dijkstra(
        start,
        |loc| loc == target,
        |(loc, depth)| {
            let mut v = loc.neighbors4().into_iter()
                .filter(|c| matches!(map.grid.get_c(*c), Cell::Open | Cell::OuterPortal(_) | Cell::InnerPortal(_)))
                .map(|c| ((c, depth), 1))
                .collect::<Vec<_>>();
            match map.grid.get_c(loc) {
                Cell::OuterPortal(_) if part2 && depth == 0 => {},
                Cell::OuterPortal(label) => {
                    if let Some(p) = map.inners.get(&label) {
                        if part2 {
                            v.push(((*p, depth - 1), 1));
                        }
                        else {
                            v.push(((*p, 0), 1));
                        }
                    }
                },
                Cell::InnerPortal(label) => {
                    if let Some(p) = map.outers.get(&label) {
                        if part2 {
                            v.push(((*p, depth + 1), 1));
                        }
                        else {
                            v.push(((*p, 0), 1));
                        }
                    }
                },
                _ => {},
            }
            v
        },
    ).unwrap()
}

fn part1(input: &[String]) -> usize {
    traverse(input, false)
}

fn part2(input: &[String]) -> usize {
    traverse(input, true)
}

fn main() {
    let input: Vec<String> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day20_test() {
        let input: Vec<String> = test_input(include_str!("day20.testinput1"));
        assert_eq!(part1(&input), 58);
        let input: Vec<String> = test_input(include_str!("day20.testinput2"));
        assert_eq!(part2(&input), 396);
    }
}
