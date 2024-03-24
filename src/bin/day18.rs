use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::ops::Add;
use std::vec::Vec;
use itertools::Itertools;
use num::Zero;
use ya_advent_lib::algorithm::dijkstra;
use ya_advent_lib::coords::Coord2D;
use ya_advent_lib::grid::Grid;
use ya_advent_lib::read::read_input;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Cell {
    Open,
    Wall,
    Key(char),
    Door(char),
    Start,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '.' => Cell::Open,
            'A'..='Z' => Cell::Door(c.to_ascii_lowercase()),
            'a'..='z' => Cell::Key(c),
            '@' => Cell::Start,
            _ => Cell::Wall,
        }
    }
}

fn merge_doors(a: &str, b: &str) -> String {
    a.chars()
        .chain(b.chars())
        .sorted()
        .dedup()
        .collect()
}

struct Edge {
    node_a: char,
    node_b: char,
    steps: usize,
    doors: String,
}
impl Edge {
    fn can_traverse(&self, keys: &str) -> bool {
        let doors: HashSet<char> = self.doors.chars().collect();
        let keys: HashSet<char> = keys.chars().collect();
        doors.difference(&keys).count() == 0
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Cost {
    steps: usize,
    doors: String,
}
impl Cost {
    fn new(steps: usize, doors: &str) -> Self {
        Cost {steps, doors: doors.to_owned()}
    }
}
impl Add for Cost {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Cost {
            steps: self.steps + other.steps,
            doors: merge_doors(&self.doors, &other.doors),
        }
    }
}
impl Zero for Cost {
    fn zero() -> Self {
        Cost { steps: 0, doors: String::new() }
    }
    fn is_zero(&self) -> bool {
        self.steps == 0 && self.doors.is_empty()
    }
}

fn mk_edges(grid: &Grid<Cell>) -> Vec<Edge> {
    let locs: HashMap<char, Coord2D> = HashMap::from_iter(
        grid.iter_with_coord()
        .filter_map(|(c,x,y)| match c {
            Cell::Key(k) => Some((k, Coord2D::new(x, y))),
            Cell::Start => Some(('@', Coord2D::new(x, y))),
            _ => None,
        })
    );
    locs.keys()
        .tuple_combinations()
        .filter_map(|(a,b)| {
            let start = locs[a];
            let target = locs[b];
            let mut queue: BinaryHeap<(Reverse<usize>, Coord2D)> = BinaryHeap::new();
            let mut traversed: HashMap<Coord2D, (usize, String)> = HashMap::new();
            queue.push((Reverse(0), start));
            traversed.insert(start, (0, String::new()));
            while let Some((_, loc)) = queue.pop() {
                let node = &traversed[&loc];
                if loc == target {
                    return Some((node.0, node.1.clone(), *a, *b));
                }
                loc.neighbors4().into_iter()
                    .filter_map(|n| match grid.get_c(n) {
                        Cell::Wall => None,
                        Cell::Key(k) if k != *b => None,
                        Cell::Open |
                        Cell::Start |
                        Cell::Key(_) => Some((n, node.0 + 1, node.1.clone())),
                        Cell::Door(d) => {
                            let doors: String = merge_doors(&node.1, &d.to_string());
                            Some((n, node.0 + 1, doors))
                        },
                    })
                    .for_each(|_| {
                        // TODO
                    });
            }
            None
        }).map(|(steps, doors, a, b)| Edge{
            node_a: a,
            node_b: b,
            steps,
            doors,
        })
        .collect()
}

fn part1(input: &[String]) -> usize {
    let grid = Grid::from_input(input, Cell::Wall, 0);
    let edges = mk_edges(&grid);
    println!("{} edges", edges.len());
    let start: (String, char) = (String::new(), '@');
    let target: String = grid.iter().filter_map(|c| match c {
            Cell::Key(k) => Some(k),
            _ => None
        }).collect();
    dijkstra(
        start,
        |(keys, _)| *keys == target,
        |(keys, node)| {
            edges.iter()
                .filter_map(|e| if e.node_a == *node {
                        Some((e, e.node_b))
                    } else if e.node_b == *node {
                        Some((e, e.node_a))
                    } else { None })
                .filter(|(edge, next)| !keys.contains(*next) && edge.can_traverse(keys))
                .map(|(edge, next)| (
                        (merge_doors(keys, &String::from(*node)), next),
                        edge.steps,
                    )
                )
                .collect()
        }
    ).unwrap()
}

fn part2(input: &[String]) -> i64 {
    0
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
    fn day18_test() {
        let input: Vec<String> = test_input(
"########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################
");
        assert_eq!(part1(&input), 132);

        let input: Vec<String> = test_input(
"#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################
");
        assert_eq!(part1(&input), 136);

        let input: Vec<String> = test_input(
"########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################
");
        assert_eq!(part1(&input), 81);

    }
}
