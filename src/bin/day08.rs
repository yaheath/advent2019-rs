use std::str::FromStr;
use std::vec::Vec;
use ya_advent_lib::read::read_input;

struct Sif<const W: usize, const H: usize> {
    pixels: Vec<u8>,
}

impl<const W: usize, const H: usize> FromStr for Sif<W, H> {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pixels: Vec<u8> = s.chars().map(|c| (c as u8) - b'0').collect();
        if pixels.len() % (W * H) != 0 {
            Err(format!("Pixel count {} not a multiple of {W}x{H}", pixels.len()))
        }
        else {
            Ok(Self{pixels})
        }
    }
}
impl <const W: usize, const H: usize> Sif<W, H> {
    fn part1(&self) -> usize {
        self.pixels.chunks(W*H)
            .min_by_key(|layer| layer.iter().filter(|p| **p == 0).count())
            .map(|layer| layer.iter().fold((0, 0), |(ones,twos), p| match p {
                1 => (ones + 1, twos),
                2 => (ones, twos + 1),
                _ => (ones, twos)
            }))
            .map(|(ones, twos)| ones * twos)
            .unwrap()
    }
    fn part2(&self) -> Vec<u8> {
        self.pixels.chunks(W*H).rev()
            .fold(vec![2; W*H], |mut v, layer| {
                v.iter_mut()
                    .zip(layer)
                    .for_each(|(vp, lp)| {
                        if *lp != 2 {
                            *vp = *lp;
                        }
                    });
                v
            })
    }
}

fn part1(input: &Sif<25, 6>) -> usize {
    input.part1()
}

fn part2(input: &Sif<25, 6>) {
    input.part2().chunks(25)
        .for_each(|v| {
            println!("{}",
                v.iter().map(|p| match p { 0 => ' ', 1 => '\u{2588}', _ => panic!() })
                .collect::<String>()
            );
        });
}

fn main() {
    let input: Vec<Sif<25, 6>> = read_input();
    println!("Part 1: {}", part1(&input[0]));
    println!("Part 2:");
    part2(&input[0]);
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day08_test() {
        let input: Vec<Sif<3, 2>> = test_input("123456789012");
        assert_eq!(input[0].part1(), 1);

        let input: Vec<Sif<2, 2>> = test_input("0222112222120000");
        assert_eq!(input[0].part2(), [0, 1, 1, 0]);
    }
}
