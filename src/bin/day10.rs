use std::cmp::Ordering;
use std::collections::{BTreeSet, BTreeMap, HashSet};
use std::f64::consts::PI;
use std::vec::Vec;
use num::integer::gcd;
use ya_advent_lib::infinite_grid::InfiniteGrid;
use ya_advent_lib::coords::Coord2D;
use ya_advent_lib::read::read_input;

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Slope {
    x: i64,
    y: i64,
}
impl From<Coord2D> for Slope {
    fn from(c: Coord2D) -> Self {
        slope_with_d(c).0
    }
}
impl Slope {
    fn angle(&self) -> f64 {
        // atan2 is normally y.atan2(x). However, we want 0 to point up, where up is -y,
        // and increasing angles are clockwise.
        let a = (self.x as f64).atan2(-self.y as f64);
        if a < 0. {
            a + 2. * PI
        }
        else {
            a
        }
    }
}
impl Ord for Slope {
    fn cmp(&self, other: &Self) -> Ordering {
        self.angle().partial_cmp(&other.angle()).unwrap()
    }
}
impl PartialOrd for Slope {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.angle().partial_cmp(&other.angle())
    }
}

fn slope_with_d(c: Coord2D) -> (Slope, i64) {
    match (c.x, c.y) {
        (0, 0) => (Slope{x:0, y:0}, 0),
        (n, 0) => (Slope{x:n.signum(), y:0}, n),
        (0, n) => (Slope{x:0, y:n.signum()}, n),
        (x, y) => {
            let d = gcd(x, y);
            (Slope{x: x/d, y: y/d}, d)
        }
    }
}

fn mkset(input: &[String]) -> HashSet<Coord2D> {
    let grid = InfiniteGrid::from_input(input, false, |c,_,_| if c == '#' { Some(true) } else { None } );
    grid.iter().map(|((x, y),_)| Coord2D::new(*x, *y)).collect()
}

fn find_best(roids: &HashSet<Coord2D>) -> (Coord2D, usize) {
    roids.iter()
        .map(|a| {
            let slopes: HashSet<Slope> = roids.iter()
                .filter(|b| *a != **b)
                .map(|b| (*b - *a).into())
                .collect();
            (*a, slopes.len())
        })
        .max_by_key(|(_,c)| *c)
        .unwrap()
}

fn vaporize(roids: &HashSet<Coord2D>, center: Coord2D) -> Vec<Coord2D> {
    let mut grouped: BTreeMap<Slope, BTreeSet<(i64, Coord2D)>> = BTreeMap::new();
    roids.iter()
        .filter(|r| **r != center)
        .for_each(|r| {
            let (s, d) = slope_with_d(*r - center);
            grouped.entry(s)
                .and_modify(|e| {e.insert((d, *r));})
                .or_insert(BTreeSet::from_iter([(d, *r)]));
        });
    let mut out = Vec::new();
    let mut removed = true;
    while removed {
        removed = false;
        grouped.values_mut()
            .for_each(|set| {
                if let Some((_, c)) = set.pop_first() {
                    removed = true;
                    out.push(c);
                }
            });
    }
    out
}

fn bothparts(input: &[String]) -> (usize, i64) {
    let roids = mkset(input);
    let (best, dist) = find_best(&roids);
    let v = vaporize(&roids, best);
    (dist, v[199].x * 100 + v[199].y)
}

fn main() {
    let input: Vec<String> = read_input();
    /*
    for (x,y) in [(0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1)] {
        let s = Slope{x, y};
        println!("{}", s.angle());
    }
    */
    let (part1, part2) = bothparts(&input);
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day10_test() {
        let input: Vec<String> = test_input(include_str!("day10.testinput"));
        let (part1, part2) = bothparts(&input);
        assert_eq!(part1, 210);
        assert_eq!(part2, 802);
    }
}
