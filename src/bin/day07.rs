use itertools::Itertools;
use std::vec::Vec;
use ya_advent_lib::read::read_input;
extern crate advent2019;
use advent2019::intcode::{IntcodeVM, ProgMem, RunErr};

/*
struct Amp<'a> {
    name: &'a str,
    program: &'a ProgMem,
}
impl<'a> Amp<'a> {
    fn new(name: &'a str, program: &'a ProgMem) -> Self {
        Self {
            name,
            program,
        }
    }
    fn start(&mut self, output_sender: mpsc::Sender<i64>, input_receiver: mpsc::Receiver<i64>) {
        let mut vm = IntcodeVM::with_mem(self.program);
        thread::Builder::new().name(self.name.to_owned()).spawn(move || {
            vm.run_with_cb(
                &mut || input_receiver.recv().unwrap(),
                &mut |v| {output_sender.send(v).unwrap();},
            ).unwrap();
            println!("Thread {} done", thread::current().name().unwrap());
        }).unwrap();
    }
}

fn run_with_phases(program: &ProgMem, phases: &[i64]) -> i64 {
    let mut a: Amp = Amp::new("a", program);
    let mut b: Amp = Amp::new("b", program);
    let mut c: Amp = Amp::new("c", program);
    let mut d: Amp = Amp::new("d", program);
    let mut e: Amp = Amp::new("e", program);
    let (in_tx, a_rx) = mpsc::channel();
    in_tx.send(phases[0]).unwrap();
    let (a_tx, b_rx) = mpsc::channel();
    a_tx.send(phases[1]).unwrap();
    let (b_tx, c_rx) = mpsc::channel();
    b_tx.send(phases[2]).unwrap();
    let (c_tx, d_rx) = mpsc::channel();
    c_tx.send(phases[3]).unwrap();
    let (d_tx, e_rx) = mpsc::channel();
    d_tx.send(phases[4]).unwrap();
    let (e_tx, out_rx) = mpsc::channel();
    a.start(a_tx.clone(), a_rx);
    b.start(b_tx.clone(), b_rx);
    c.start(c_tx.clone(), c_rx);
    d.start(d_tx.clone(), d_rx);
    e.start(e_tx.clone(), e_rx);
    in_tx.send(0).unwrap();
    if phases[0] > 4 {
        let v = out_rx.recv().unwrap();
        in_tx.send(v).unwrap();
    }
    out_rx.recv().unwrap()
}
*/

fn run_with_phases(program: &ProgMem, phases: &[i64]) -> i64 {
    let mut vms = Vec::new();
    for p in phases {
        let mut vm = IntcodeVM::with_mem(program);
        vm.input_queue.push_back(*p);
        vms.push(vm);
    }

    let mut nextv = 0;
    let mut repeat = true;
    while repeat {
        repeat = false;
        for vm in vms.iter_mut() {
            vm.input_queue.push_back(nextv);
            match vm.run_with_cb(&mut || None, &mut |v| {
                nextv = v;
            }) {
                Ok(_) => {}
                Err(RunErr::InputNeeded) => {
                    repeat = true;
                }
                _ => panic!(),
            }
        }
    }
    nextv
}

fn part1(input: &ProgMem) -> i64 {
    (0..5)
        .permutations(5)
        .map(|v| run_with_phases(input, &v))
        .max()
        .unwrap()
}

fn part2(input: &ProgMem) -> i64 {
    (5..10)
        .permutations(5)
        .map(|v| run_with_phases(input, &v))
        .max()
        .unwrap()
}

fn main() {
    let input: Vec<ProgMem> = read_input();
    println!("Part 1: {}", part1(&input[0]));
    println!("Part 2: {}", part2(&input[0]));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day07_test() {
        let input: Vec<ProgMem> = test_input("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
        assert_eq!(part1(&input[0]), 43210);
        let input: Vec<ProgMem> =
            test_input("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0");
        assert_eq!(part1(&input[0]), 54321);
        let input: Vec<ProgMem> = test_input("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0");
        assert_eq!(part1(&input[0]), 65210);
    }
}
