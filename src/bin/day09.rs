use std::vec::Vec;

use ya_advent_lib::read::read_input;
extern crate advent2019;
use advent2019::intcode::{IntcodeVM, ProgMem};

fn part1(input: &ProgMem) -> i64 {
    let mut keycode = 0;
    let mut vm = IntcodeVM::with_mem(input);
    vm.input_queue.push_back(1);
    vm.run_with_cb(&mut || None, &mut |v| keycode = v).unwrap();
    keycode
}

fn part2(input: &ProgMem) -> i64 {
    let mut outp = 0;
    let mut vm = IntcodeVM::with_mem(input);
    vm.input_queue.push_back(2);
    vm.run_with_cb(&mut || None, &mut |v| outp = v).unwrap();
    outp
}

fn main() {
    let input: Vec<ProgMem> = read_input();
    println!("Part 1: {}", part1(&input[0]));
    println!("Part 2: {}", part2(&input[0]));
}

#[cfg(test)]
mod tests {
    use super::*;
    

    #[test]
    fn day07_test() {
        let progmem = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99".parse::<ProgMem>().unwrap();
        let mut vm = IntcodeVM::with_mem(&progmem);
        let mut outv: Vec<i64> = Vec::new();
        vm.run_with_cb(&mut || None, &mut |v| outv.push(v)).unwrap();
        assert_eq!(outv, [109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99]);

        let progmem = "1102,34915192,34915192,7,4,7,99,0".parse::<ProgMem>().unwrap();
        let mut vm = IntcodeVM::with_mem(&progmem);
        let mut outv: Vec<i64> = Vec::new();
        vm.run_with_cb(&mut || None, &mut |v| outv.push(v)).unwrap();
        assert_eq!(outv, [1219070632396864]);

        let progmem = "104,1125899906842624,99".parse::<ProgMem>().unwrap();
        let mut vm = IntcodeVM::with_mem(&progmem);
        let mut outv: Vec<i64> = Vec::new();
        vm.run_with_cb(&mut || None, &mut |v| outv.push(v)).unwrap();
        assert_eq!(outv, [1125899906842624]);
    }
}
