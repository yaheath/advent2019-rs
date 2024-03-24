use itertools::Itertools;
use std::collections::HashSet;
use std::vec::Vec;
use ya_advent_lib::coords::Coord2D;
use ya_advent_lib::read::read_input;

struct BugMap {
    bugs: HashSet<(i64, i64, i64)>,
    min_z: i64,
    max_z: i64,
}
impl BugMap {
    fn from_input(input: &[String]) -> Self {
        let mut bugs = HashSet::new();
        for (y, row) in input.iter().enumerate() {
            for (x, c) in row.chars().enumerate() {
                if c == '#' {
                    bugs.insert((x as i64, y as i64, 0));
                }
            }
        }
        BugMap {
            bugs,
            min_z: 0,
            max_z: 0,
        }
    }

    fn bio_rating(&self) -> u64 {
        (0..5)
            .cartesian_product(0..5)
            .enumerate()
            .filter_map(|(idx, (y, x))| {
                if self.bugs.contains(&(x, y, 0)) {
                    Some(2u64.pow(idx as u32))
                } else {
                    None
                }
            })
            .sum()
    }

    fn step(&mut self) {
        let mut next = HashSet::new();
        (0..5).cartesian_product(0..5).for_each(|(x, y)| {
            let occupied = self.bugs.contains(&(x, y, 0));
            match (self.neighbors(x, y), occupied) {
                (1, true) => {
                    next.insert((x, y, 0));
                }
                (1, false) | (2, false) => {
                    next.insert((x, y, 0));
                }
                _ => {}
            }
        });
        self.bugs = next;
    }

    fn neighbors(&self, x: i64, y: i64) -> usize {
        let c = Coord2D::new(x, y);
        c.neighbors4()
            .into_iter()
            .filter(|c| self.bugs.contains(&(c.x, c.y, 0)))
            .count()
    }

    #[allow(dead_code)]
    fn print(&self) {
        for y in 0..5 {
            for x in 0..5 {
                if self.bugs.contains(&(x, y, 0)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn step2(&mut self) {
        let mut next = HashSet::new();
        let z_range = self.min_z - 1..=self.max_z + 1;
        (0..5)
            .cartesian_product(0..5)
            .filter(|(x, y)| !(*x == 2 && *y == 2))
            .cartesian_product(z_range)
            .for_each(|((x, y), z)| {
                let occupied = self.bugs.contains(&(x, y, z));
                match (self.neighbors2(x, y, z), occupied) {
                    (1, true) => {
                        next.insert((x, y, z));
                    }
                    (1, false) | (2, false) => {
                        next.insert((x, y, z));
                        if z < self.min_z {
                            self.min_z = z;
                        } else if z > self.max_z {
                            self.max_z = z;
                        }
                    }
                    _ => {}
                }
            });
        self.bugs = next;
    }

    fn neighbors2(&self, x: i64, y: i64, z: i64) -> usize {
        let mut count = 0;

        // left neighbor(s)
        count += match (x, y) {
            (0, _) => {
                if self.bugs.contains(&(1, 2, z - 1)) {
                    1
                } else {
                    0
                }
            }
            (3, 2) => (0..5)
                .filter(|yy| self.bugs.contains(&(4, *yy, z + 1)))
                .count(),
            _ => {
                if self.bugs.contains(&(x - 1, y, z)) {
                    1
                } else {
                    0
                }
            }
        };

        // right neighbor(s)
        count += match (x, y) {
            (4, _) => {
                if self.bugs.contains(&(3, 2, z - 1)) {
                    1
                } else {
                    0
                }
            }
            (1, 2) => (0..5)
                .filter(|yy| self.bugs.contains(&(0, *yy, z + 1)))
                .count(),
            _ => {
                if self.bugs.contains(&(x + 1, y, z)) {
                    1
                } else {
                    0
                }
            }
        };

        // top neighbor(s)
        count += match (x, y) {
            (_, 0) => {
                if self.bugs.contains(&(2, 1, z - 1)) {
                    1
                } else {
                    0
                }
            }
            (2, 3) => (0..5)
                .filter(|xx| self.bugs.contains(&(*xx, 4, z + 1)))
                .count(),
            _ => {
                if self.bugs.contains(&(x, y - 1, z)) {
                    1
                } else {
                    0
                }
            }
        };

        // bottom neighbor(s)
        count += match (x, y) {
            (_, 4) => {
                if self.bugs.contains(&(2, 3, z - 1)) {
                    1
                } else {
                    0
                }
            }
            (2, 1) => (0..5)
                .filter(|xx| self.bugs.contains(&(*xx, 0, z + 1)))
                .count(),
            _ => {
                if self.bugs.contains(&(x, y + 1, z)) {
                    1
                } else {
                    0
                }
            }
        };

        count
    }
}

fn part1(input: &[String]) -> u64 {
    let mut layouts: HashSet<u64> = HashSet::new();
    let mut bugmap = BugMap::from_input(input);
    layouts.insert(bugmap.bio_rating());
    loop {
        bugmap.step();
        let r = bugmap.bio_rating();
        if layouts.contains(&r) {
            return r;
        }
        layouts.insert(r);
    }
}

fn part2(input: &[String]) -> usize {
    let mut bugmap = BugMap::from_input(input);
    for _ in 0..200 {
        bugmap.step2();
    }
    bugmap.bugs.len()
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
    fn day24_test() {
        let input: Vec<String> = test_input(
            "....#
#..#.
#..##
..#..
#....
",
        );
        assert_eq!(part1(&input), 2129920);

        let mut bugmap = BugMap::from_input(&input);
        for _ in 0..10 {
            bugmap.step2();
        }
        assert_eq!(bugmap.bugs.len(), 99);
    }
}
