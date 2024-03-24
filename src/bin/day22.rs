use std::str::FromStr;
use std::vec::Vec;
use ya_advent_lib::read::read_input;

enum Action {
    Reverse,    // deal into new stack
    Rotate(i64), // cut
    Multiply(i64), // deal with increment
}

impl FromStr for Action {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words = s.split(' ').collect::<Vec<_>>();
        if words[0] == "cut" {
            let n = words[1].parse::<i64>().unwrap();
            Ok(Action::Rotate(n))
        }
        else if words[1] == "into" {
            Ok(Action::Reverse)
        }
        else if words[1] == "with" {
            let n = words[words.len() - 1].parse::<i64>().unwrap();
            Ok(Action::Multiply(n))
        }
        else {
            Err(())
        }
    }
}

fn part1(input: &[Action]) -> i64 {
    let ncards = 10007;
    input.iter().fold(2019, |card, i| match i {
        Action::Reverse => ncards - 1 - card,
        Action::Rotate(n) => {
            let mut nextcard = card - n;
            if nextcard < 0 { nextcard += ncards; }
            nextcard % ncards
        },
        Action::Multiply(n) => (card * n) % ncards
    })
}

fn mod_pow(base: i128, exp: i128, modulus: i128) -> i128 {
    if modulus == 1 { return 0 }
    let mut result = 1;
    let mut base = base % modulus;
    let mut exp = exp;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp >>= 1;
        base = base * base % modulus
    }
    result
}

fn part2(input: &[Action]) -> i64 {
    // I cribbed this algorithm from Reddit. I don't actually
    // fully understand it.
    let ncards: i128 = 119315717514047;
    let nshuffles: i128 = 101741582076661;
    let (a, b) = input.iter().rev().fold((1, 0), |(a, b), i| match i {
        Action::Reverse => (-a % ncards, (-b - 1) % ncards),
        Action::Rotate(n) => (a, (b + *n as i128) % ncards),
        Action::Multiply(n) => {
            let n = mod_pow(*n as i128, ncards - 2, ncards);
            (a * n % ncards, b * n % ncards)
        },
    });
    let a_pow = mod_pow(a, nshuffles, ncards);
    let x = b * ((a_pow - 1) * mod_pow(a-1, ncards-2, ncards) % ncards) % ncards;
    ((2020 * a_pow % ncards + x) % ncards) as i64
}

fn main() {
    let input: Vec<Action> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
