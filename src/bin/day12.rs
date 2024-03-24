use std::str::FromStr;
use std::vec::Vec;
use itertools::Itertools;
use lazy_static::lazy_static;
use num::integer::lcm;
use regex::Regex;
use ya_advent_lib::coords::Coord3D;
use ya_advent_lib::read::read_input;

#[derive(Copy, Clone)]
struct Moon {
    pos: Coord3D,
    initial_pos: Coord3D,
    vel: Coord3D,
}

impl FromStr for Moon {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"x=(-?\d+), y=(-?\d+), z=(-?\d+)"
            ).unwrap();
        }
        if let Some(caps) = RE.captures(s) {
            let x:i64 = caps.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let y:i64 = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
            let z:i64 = caps.get(3).unwrap().as_str().parse::<i64>().unwrap();
            Ok(Moon{
                pos: Coord3D::new(x, y, z),
                initial_pos: Coord3D::new(x, y, z),
                vel: Coord3D::new(0, 0, 0),
            })
        }
        else {
            Err(())
        }
    }
}
impl Moon {
    fn energy(&self) -> i64 {
        let origin = Coord3D::new(0, 0, 0);
        self.pos.mdist_to(&origin) * self.vel.mdist_to(&origin)
    }
}

fn simulate(moons: &mut [Moon], steps: usize) {
    for _ in 0..steps {
    let mut dv = vec![Coord3D::new(0, 0, 0); moons.len()];
        moons.iter().enumerate()
            .tuple_combinations()
            .for_each(|((a_ix, a), (b_ix, b))| {
                if a.pos.x < b.pos.x {
                    dv[a_ix].x += 1;
                    dv[b_ix].x -= 1;
                }
                else if a.pos.x > b.pos.x {
                    dv[a_ix].x -= 1;
                    dv[b_ix].x += 1;
                }
                if a.pos.y < b.pos.y {
                    dv[a_ix].y += 1;
                    dv[b_ix].y -= 1;
                }
                else if a.pos.y > b.pos.y {
                    dv[a_ix].y -= 1;
                    dv[b_ix].y += 1;
                }
                if a.pos.z < b.pos.z {
                    dv[a_ix].z += 1;
                    dv[b_ix].z -= 1;
                }
                else if a.pos.z > b.pos.z {
                    dv[a_ix].z -= 1;
                    dv[b_ix].z += 1;
                }
            });
        moons.iter_mut().enumerate()
            .for_each(|(idx, m)| {
                m.vel += dv[idx];
                m.pos += m.vel;
            });
    }
}

fn part1(input: &[Moon]) -> i64 {
    let mut moons: Vec<Moon> = input.to_vec();
    simulate(&mut moons, 1000);
    moons.iter().map(|m| m.energy()).sum()
}

fn find_repeat(input: &[Moon]) -> usize {
    let mut moons: Vec<Moon> = input.to_vec();
    let mut xrepeat = 0usize;
    let mut yrepeat = 0usize;
    let mut zrepeat = 0usize;

    for step in 1.. {
        simulate(&mut moons, 1);
        let (xr, yr, zr) = moons.iter()
            .fold((0, 0, 0), |(xr, yr, zr), moon| (
                if moon.pos.x == moon.initial_pos.x && moon.vel.x == 0 {xr + 1} else {xr},
                if moon.pos.y == moon.initial_pos.y && moon.vel.y == 0 {yr + 1} else {yr},
                if moon.pos.z == moon.initial_pos.z && moon.vel.z == 0 {zr + 1} else {zr},
            ));
        if xrepeat == 0 && xr == moons.len() { xrepeat = step; }
        if yrepeat == 0 && yr == moons.len() { yrepeat = step; }
        if zrepeat == 0 && zr == moons.len() { zrepeat = step; }
        if xrepeat > 0 && yrepeat > 0 && zrepeat > 0 {
            break;
        }
    }
    lcm(lcm(xrepeat, yrepeat), zrepeat)
}

fn part2(input: &[Moon]) -> usize {
    find_repeat(input)
}

fn main() {
    let input: Vec<Moon> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day12_test() {
        let mut moons: Vec<Moon> = test_input(
"<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>
");
        let mut moons2 = moons.clone();
        simulate(&mut moons, 10);
        let energy: i64 = moons.iter().map(|m| m.energy()).sum();
        assert_eq!(energy, 179);
        assert_eq!(find_repeat(&mut moons2), 2772);

        let mut moons: Vec<Moon> = test_input(
"<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>
");
        let mut moons2 = moons.clone();
        simulate(&mut moons, 100);
        let energy: i64 = moons.iter().map(|m| m.energy()).sum();
        assert_eq!(energy, 1940);
        assert_eq!(find_repeat(&mut moons2), 4686774924);
    }
}
