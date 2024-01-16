use std::str::FromStr;
use std::ops::RangeInclusive;
use std::vec::Vec;
use itertools::Itertools;
use ya_advent_lib::coords::{CDir, Coord2D};
use ya_advent_lib::read::read_input;

#[derive(Copy, Clone, Debug)]
struct Seg {
    dir: CDir,
    steps: i64,
}
impl FromStr for Seg {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dir = match s.chars().next().unwrap() {
            'U' => CDir::N,
            'R' => CDir::E,
            'L' => CDir::W,
            'D' => CDir::S,
            _ => panic!(),
        };
        let steps = s[1..].parse::<i64>().unwrap();
        Ok(Self{dir, steps})
    }
}
impl From<&Seg> for Coord2D {
    fn from(value: &Seg) -> Self {
        (Coord2D::new(0, 0) + value.dir) * value.steps
    }
}

struct WirePath(Vec<Seg>);

impl FromStr for WirePath {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
                s.split(',').flat_map(|ss| ss.parse::<Seg>()).collect()
        ))
    }
}

#[derive(Clone, Debug)]
struct Segment {
    a: Coord2D,
    b: Coord2D,
}
impl Segment {
    fn from_wirepath(wp: &WirePath) -> Vec<Segment> {
        let mut point = Coord2D::new(0, 0);
        let mut segments = Vec::with_capacity(wp.0.len());
        for seg in &wp.0 {
            let nextpoint = point + seg;
            segments.push(Segment{a: point, b: nextpoint});
            point = nextpoint;
        }
        segments
    }
    fn x_range(&self) -> RangeInclusive<i64> {
        self.a.x.min(self.b.x) ..= self.a.x.max(self.b.x)
    }
    fn y_range(&self) -> RangeInclusive<i64> {
        self.a.y.min(self.b.y) ..= self.a.y.max(self.b.y)
    }
    fn len(&self) -> i64 {
        self.a.mdist_to(&self.b)
    }
    // Perpendicular intersection (returns None for co-linear overlaps)
    fn p_intersect(&self, other: &Self) -> Option<Coord2D> {
        if self.a.x == self.b.x && other.a.y == other.b.y && self.y_range().contains(&other.a.y) && other.x_range().contains(&self.a.x) {
            Some(Coord2D::new(self.a.x, other.a.y))
        }
        else if self.a.y == self.b.y && other.a.x == other.b.x && self.x_range().contains(&other.a.x) && other.y_range().contains(&self.a.y) {
            Some(Coord2D::new(other.a.x, self.a.y))
        }
        else {
            None
        }
    }
}

fn part1(input: &[WirePath]) -> i64 {
    let wire_a = Segment::from_wirepath(&input[0]);
    let wire_b = Segment::from_wirepath(&input[1]);
    let origin = Coord2D::new(0, 0);
    wire_b.iter()
        .cartesian_product(wire_a)
        .filter_map(|(b, a)| a.p_intersect(b))
        .filter(|c| *c != origin)
        .map(|c| c.mdist_to(&origin))
        .min()
        .unwrap()
}

fn part2(input: &[WirePath]) -> i64 {
    let wire_a = Segment::from_wirepath(&input[0]);
    let wire_b = Segment::from_wirepath(&input[1]);
    let mut a_len = 0;
    let mut b_len = 0;
    wire_b.iter()
        .cartesian_product(wire_a.iter().enumerate())
        .filter_map(|(b, (a_idx, a))| {
            if a_idx == 0 {
                a_len = 0;
            }
            let r = a.p_intersect(b)
                .map(|c| a_len + b_len + a.a.mdist_to(&c) + b.a.mdist_to(&c));
            a_len += a.len();
            if a_idx == wire_a.len() - 1 {
                b_len += b.len();
            }
            r
        })
        .filter(|c| *c != 0)
        .min()
        .unwrap()
}

fn main() {
    let input: Vec<WirePath> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day03_test() {
        let input: Vec<WirePath> = test_input("R8,U5,L5,D3\nU7,R6,D4,L4\n");
        assert_eq!(part1(&input), 6);
        assert_eq!(part2(&input), 30);
        let input: Vec<WirePath> = test_input("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83\n");
        assert_eq!(part1(&input), 159);
        assert_eq!(part2(&input), 610);
        let input: Vec<WirePath> = test_input("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7\n");
        assert_eq!(part1(&input), 135);
        assert_eq!(part2(&input), 410);
    }
}
