use std::collections::{HashMap, HashSet};
use std::vec::Vec;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use ya_advent_lib::coords::CDir;
use ya_advent_lib::read::read_input;
extern crate advent2019;
use advent2019::intcode::{IntcodeVM, ProgMem, RunErr};

lazy_static! {
    static ref ROOM_RE: Regex = Regex::new(r"== (.*) ==").unwrap();
}
lazy_static! {
    static ref WIN_RE: Regex = Regex::new(r"in by typing (\d+)").unwrap();
}

const V: bool = false;

#[derive(Copy, Clone, Eq, PartialEq)]
enum State {
    Exploring,
    Backtrack,
    Picking,
    MoveToCheckpoint,
    Test,
}

struct Room {
    //name: String,
    exits: HashMap<CDir, String>,
}
impl Room {
    fn new() -> Self {
        Self {exits: HashMap::new() }
    }
}

fn play_game(program: &ProgMem) {
    let mut vm = IntcodeVM::with_mem(program);
    let mut rooms: HashMap<String, Room> = HashMap::new();
    let mut bad_items: HashSet<String> = HashSet::from_iter(["infinite loop".to_string()]);
    let mut current_path: Vec<CDir> = Vec::new();
    let mut inventory: HashSet<String> = HashSet::new();
    let mut path_to_checkpoint: Vec<CDir> = Vec::new();
    let mut current_room = String::new();
    let mut prev_room = String::new();
    let mut last_picked_item = String::new();
    let mut state = State::Exploring;
    let mut item_test_iter: Option<Box<dyn Iterator<Item=Vec<String>>>> = None;
    let mut dir_to_test = CDir::N;
    loop {
        let mut output = String::new();
        match vm.run_with_cb(&mut || None, &mut |v| output.push(v as u8 as char)) {
            Ok(_) => {
                assert!(!last_picked_item.is_empty() || state == State::Test);
                if V { print!("{output}"); }
                if state == State::Test {
                    if !V {
                        println!("{}", output.lines().last().unwrap());
                    }
                    return;
                }
                bad_items.insert(last_picked_item);
                last_picked_item = String::new();
                // reset
                vm = IntcodeVM::with_mem(program);
                rooms = HashMap::new();
                inventory = HashSet::new();
                current_path = Vec::new();
                current_room = String::new();
                prev_room = String::new();
                state = State::Exploring;
                if V { println!("*** Starting over ***"); }
                continue;
            },
            Err(RunErr::InputNeeded) => {},
            Err(_) => panic!(),
        }

        if V { print!("{output}"); }

        let mut line_iter = output.lines();
        let mut current_room_items: Vec<String> = Vec::new();
        let mut exits: HashSet<CDir> = HashSet::new();

        while let Some(line) = line_iter.next() {
            if let Some(caps) = ROOM_RE.captures(line) {
                current_room = caps.get(1).unwrap().as_str().into();
                exits.clear();
                if state == State::Exploring && current_room != prev_room {
                    rooms.entry(current_room.clone())
                        .or_insert(Room::new());
                    if let Some(moved_dir) = current_path.last().copied() {
                        rooms.entry(prev_room.clone())
                            .and_modify(|e| {
                                e.exits.entry(moved_dir)
                                    .and_modify(|r| *r = current_room.clone())
                                    .or_insert(current_room.clone());
                            });
                        rooms.entry(current_room.clone())
                            .and_modify(|e| {
                                e.exits.entry(-moved_dir)
                                    .and_modify(|r| *r = prev_room.clone())
                                    .or_insert(prev_room.clone());
                            });
                    }
                }
            }
            else if line.starts_with("Doors here lead") {
                while let Some(dir) = line_iter.next() {
                    match dir {
                        "- north" => {exits.insert(CDir::N);},
                        "- east" => {exits.insert(CDir::E);},
                        "- south" => {exits.insert(CDir::S);},
                        "- west" => {exits.insert(CDir::W);},
                        _ => break,
                    }
                }
            }
            else if line.starts_with("Items here") {
                while let Some(item) = line_iter.next() {
                    if !item.starts_with("- ") { break; }
                    let item = item[2..].to_string();
                    if !bad_items.contains(&item) {
                        current_room_items.push(item);
                    }
                }
            }
        }

        if state == State::Exploring && current_room.is_empty() {
            // Some hazardous items cause you to not be able to move.
            // reset
            bad_items.insert(last_picked_item);
            last_picked_item = String::new();
            vm = IntcodeVM::with_mem(program);
            rooms = HashMap::new();
            inventory = HashSet::new();
            current_path = Vec::new();
            current_room = String::new();
            prev_room = String::new();
            state = State::Exploring;
            if V { println!("*** Starting over ***"); }
            continue;
        }

        for exit in exits {
            rooms.entry(current_room.clone())
                .and_modify(|e| {
                    e.exits.entry(exit).or_insert(String::new());
                });
        }

        loop {
            match state {
                State::Exploring | State::Backtrack => {
                    if current_room == "Security Checkpoint" {
                        if path_to_checkpoint.is_empty() {
                            path_to_checkpoint = current_path.clone();
                        }
                        if prev_room == current_room {
                            // entered the pressure-sensitive floor
                            // and got kicked back
                            dir_to_test = current_path.pop().unwrap();
                        }
                    }
                    if current_room_items.len() > 0 {
                        state = State::Picking;
                        continue;
                    }
                    let room = &rooms[&current_room];
                    if let Some(nextdir) = room.exits.iter()
                        .find(|(_,v)| *v == "").map(|(k,_)| k) {
                            current_path.push(*nextdir);
                            prev_room = current_room;
                            current_room = String::new();
                            state = State::Exploring;
                            vm.ascii_input(dir_cmd(*nextdir));
                            if V { print!("{}", dir_cmd(*nextdir)); }
                    }
                    else {
                        if let Some(nextdir) = current_path.last().copied() {
                            current_path.pop();
                            prev_room = current_room;
                            current_room = String::new();
                            state = State::Backtrack;
                            vm.ascii_input(dir_cmd(-nextdir));
                            if V { print!("{}", dir_cmd(-nextdir)); }
                        }
                        else {
                            prev_room = String::new();
                            state = State::MoveToCheckpoint;
                            if V { println!("*** Finished exploring, moving to Security Checkpoint ***"); }
                            continue;
                        }
                    }
                    break;
                },
                State::Picking => {
                    if let Some(item) = current_room_items.pop() {
                        last_picked_item = item.clone();
                        inventory.insert(item.clone());
                        let cmd = format!("take {item}\n");
                        vm.ascii_input(&cmd);
                        if V { print!("{cmd}"); }
                    }
                    else {
                        state = State::Exploring;
                        continue;
                    }
                    break;
                },
                State::MoveToCheckpoint => {
                    if path_to_checkpoint.is_empty() {
                        state = State::Test;
                        item_test_iter = Some(Box::new(inventory.clone().into_iter().powerset()));
                        if V { println!("*** Trying to find the correct weight ***"); }
                        continue;
                    }
                    let next = path_to_checkpoint.splice(0..1, []).next().unwrap();
                    current_path.push(next);
                    vm.ascii_input(dir_cmd(next));
                    if V { print!("{}", dir_cmd(next)); }
                    break;
                },
                State::Test => {
                    if let Some(carry) = item_test_iter.as_deref_mut().unwrap().next() {
                        let carry: HashSet<String> = HashSet::from_iter(carry);
                        for drop in inventory.difference(&carry) {
                            let cmd = format!("drop {drop}\n");
                            vm.ascii_input(&cmd);
                            if V { print!("{cmd}"); }
                        }
                        for take in carry.difference(&inventory) {
                            let cmd = format!("take {take}\n");
                            vm.ascii_input(&cmd);
                            if V { print!("{cmd}"); }
                        }
                        inventory = carry.clone();
                        vm.ascii_input(dir_cmd(dir_to_test));
                        if V { print!("{}", dir_cmd(dir_to_test)); }
                    }
                    else {
                        panic!();
                    }
                    break;
                },
            }
        }
    }
}

fn dir_cmd(dir: CDir) -> &'static str {
    match dir {
        CDir::N => "north\n",
        CDir::E => "east\n",
        CDir::W => "west\n",
        CDir::S => "south\n",
    }
}

#[allow(dead_code)]
fn part1(input: &ProgMem) -> i64 {
    let mut vm = IntcodeVM::with_mem(&input);
    let mut out = 0;
    vm.run_interactive(&mut |v| {out = v;}).unwrap();
    out
}
fn main() {
    let input: Vec<ProgMem> = read_input();
    play_game(&input[0]);
}
