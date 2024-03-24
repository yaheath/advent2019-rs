use std::iter::repeat;
use std::str::FromStr;
use std::vec::Vec;
use ya_advent_lib::read::read_input;

struct Input {
    digits: Vec<u8>,
}

impl FromStr for Input {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let digits = s.chars().map(|c| (c as u8) - b'0').collect();
        Ok(Input { digits })
    }
}

fn fft(digits: &[u8]) -> Vec<u8> {
    (1..=digits.len())
        .map(|n| {
            repeat(0i32)
                .take(n)
                .chain(repeat(1i32).take(n))
                .chain(repeat(0i32).take(n))
                .chain(repeat(-1i32).take(n))
                .cycle()
                .skip(1)
        })
        .map(|itr| digits.iter().zip(itr).map(|(a, b)| *a as i32 * b).sum())
        .map(|val: i32| (val.abs() % 10) as u8)
        .collect()
}

fn part1(input: &Input) -> String {
    let mut vec = input.digits.to_owned();
    for _ in 0..100 {
        vec = fft(&vec);
    }
    vec.iter().take(8).map(|c| (*c + b'0') as char).collect()
}

fn part2(input: &Input) -> String {
    let offset = input
        .digits
        .iter()
        .take(7)
        .map(|b| (*b + b'0') as char)
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    let datalen = input.digits.len();
    let lenx10k = datalen * 10000;
    let mut digits: Vec<u8> = Vec::new();
    for i in offset..lenx10k {
        digits.push(input.digits[i % datalen]);
    }
    for _ in 0..100 {
        for i in (0..digits.len() - 1).rev() {
            digits[i] = (digits[i] + digits[i + 1]) % 10;
        }
    }
    digits.iter().take(8).map(|c| (*c + b'0') as char).collect()
}

fn main() {
    let input: Vec<Input> = read_input();
    println!("Part 1: {}", part1(&input[0]));
    println!("Part 2: {}", part2(&input[0]));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day16_test() {
        let input: Vec<Input> = test_input("80871224585914546619083218645595");
        assert_eq!(part1(&input[0]), "24176176");
        let input: Vec<Input> = test_input("19617804207202209144916044189917");
        assert_eq!(part1(&input[0]), "73745418");
        let input: Vec<Input> = test_input("69317163492948606335995924319873");
        assert_eq!(part1(&input[0]), "52432133");
        let input: Vec<Input> = test_input("03036732577212944063491565474664");
        assert_eq!(part2(&input[0]), "84462026");
        let input: Vec<Input> = test_input("02935109699940807407585447034323");
        assert_eq!(part2(&input[0]), "78725270");
        let input: Vec<Input> = test_input("03081770884921959731165446850517");
        assert_eq!(part2(&input[0]), "53553731");
    }
}
