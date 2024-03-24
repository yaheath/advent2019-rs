use std::collections::{HashMap, VecDeque};
use std::str::FromStr;
use std::vec::Vec;
use ya_advent_lib::read::read_input;

#[derive(Clone)]
struct Chem {
    name: String,
    qty: usize,
}
impl FromStr for Chem {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (q, name) = s.split_once(' ').unwrap();
        let name = name.to_string();
        let qty = q.parse::<usize>().unwrap();
        Ok(Self{name, qty})
    }
}

#[derive(Clone)]
struct Reaction {
    ingredients: Vec<Chem>,
    product: Chem,
}

impl FromStr for Reaction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (ing, pr) = s.split_once(" => ").unwrap();
        let ingredients = ing.split(", ").map(|i| i.parse::<Chem>().unwrap()).collect();
        let product = pr.parse::<Chem>().unwrap();

        Ok(Self{ingredients, product})
    }
}

fn mk_table(input: &[Reaction]) -> HashMap<&str, &Reaction> {
    input.iter()
        .map(|i| (i.product.name.as_str(), i))
        .collect()
}

fn search(table: &HashMap<&str, &Reaction>, n_fuel: usize) -> usize {
    let mut queue = VecDeque::new();
    let target = Chem{name: "FUEL".to_string(), qty: n_fuel};
    queue.push_back(target);
    let mut n_ore = 0;
    let mut inventory: HashMap<String, usize> = HashMap::new();

    while let Some(target) = queue.pop_front() {
        let mut needed = target.qty;
        inventory.entry(target.name.clone())
            .and_modify(|e| {
                if *e >= needed {
                    *e -= needed;
                    needed = 0;
                }
                else {
                    needed -= *e;
                    *e = 0;
                }
            });
        if needed == 0 { continue; }
        let reaction = table[target.name.as_str()];
        let num_per_batch = reaction.product.qty;
        let num_batches = (needed + num_per_batch - 1) / num_per_batch;
        let num_produced = num_batches * num_per_batch;
        inventory.entry(target.name.clone())
            .and_modify(|e| *e += num_produced - needed)
            .or_insert(num_produced - needed);
        for chem in &reaction.ingredients {
            let num_required = chem.qty * num_batches;
            if chem.name == "ORE" {
                n_ore += num_required;
            }
            else {
                queue.push_back(Chem{name: chem.name.clone(), qty: num_required});
            }
        }
    }
    n_ore
}

fn part1(input: &[Reaction]) -> usize {
    let table = mk_table(input);
    search(&table, 1)
}

fn part2(input: &[Reaction]) -> usize {
    let table = mk_table(input);
    let target = 1000000000000;
    let mut est = target / (search(&table, 10000) / 1000);
    let interval = est / 10;
    let mut lowerbound = 0;
    let mut upperbound = 0;
    loop {
        let ore = search(&table, est);
        if ore == target {
            return est;
        }
        if ore < target {
            if lowerbound == 0 {
                lowerbound = est;
                est = if upperbound > 0 {
                    lowerbound + (upperbound - lowerbound) / 2
                } else {
                    est + interval
                };
            }
            else if upperbound == 0 {
                est += interval;
            }
            else {
                lowerbound = est;
                est = lowerbound + (upperbound - lowerbound) / 2;
                if est == lowerbound {
                    return est;
                }
            }
        }
        else if upperbound == 0 {
            upperbound = est;
            est = if lowerbound > 0 {
                lowerbound + (upperbound - lowerbound) / 2
            } else {
                est - interval
            };
        }
        else if lowerbound == 0 {
            est -= interval;
        }
        else {
            upperbound = est;
            est = lowerbound + (upperbound - lowerbound) / 2;
            if est == lowerbound {
                return est;
            }
        }
    }
}

fn main() {
    let input: Vec<Reaction> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day14_test() {
        let input: Vec<Reaction> = test_input(
"10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL
");
        assert_eq!(part1(&input), 31);

        let input: Vec<Reaction> = test_input(
"9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL
");
        assert_eq!(part1(&input), 165);

        let input: Vec<Reaction> = test_input(
"157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT
");
        assert_eq!(part1(&input), 13312);
        assert_eq!(part2(&input), 82892753);

        let input: Vec<Reaction> = test_input(
"2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF
");
        assert_eq!(part1(&input), 180697);
        assert_eq!(part2(&input), 5586022);

        let input: Vec<Reaction> = test_input(
"171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX
");
        assert_eq!(part1(&input), 2210736);
        assert_eq!(part2(&input), 460664);
    }
}
