use std::collections::VecDeque;
use std::str::FromStr;
use std::vec::Vec;

pub struct ProgMem(pub Vec<i64>);

impl FromStr for ProgMem {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(
            Self(
                s.split(',').map(|ss| ss.parse::<i64>().unwrap()).collect()
            )
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StepResult {
    Ok,
    Halt,
    InputNeeded,
    InvalidInstr(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RunErr {
    InputNeeded,
    InvalidInstr(String),
}

enum Opcode {
    Add = 1,
    Mul,
    Inp,
    Out,
    Jnz,
    Jz,
    Lt,
    Eq,
    Rlb,
    Hlt = 99,
}
impl Opcode {
    fn size(&self) -> usize {
        match self {
            Self::Add => 4,
            Self::Mul => 4,
            Self::Inp => 2,
            Self::Out => 2,
            Self::Jnz => 3,
            Self::Jz => 3,
            Self::Lt => 4,
            Self::Eq => 4,
            Self::Rlb => 2,
            Self::Hlt => 1,
        }
    }
    fn stores_to(&self, argnum: usize) -> bool {
        match self {
            Self::Add if argnum == 2 => true,
            Self::Mul if argnum == 2 => true,
            Self::Inp if argnum == 0 => true,
            Self::Lt if argnum == 2 => true,
            Self::Eq if argnum == 2 => true,
            _ => false,
        }
    }
}
impl TryFrom<i64> for Opcode {
    type Error = String;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        let op = v % 100;
        match op {
            1 => Ok(Self::Add),
            2 => Ok(Self::Mul),
            3 => Ok(Self::Inp),
            4 => Ok(Self::Out),
            5 => Ok(Self::Jnz),
            6 => Ok(Self::Jz),
            7 => Ok(Self::Lt),
            8 => Ok(Self::Eq),
            9 => Ok(Self::Rlb),
            99 => Ok(Self::Hlt),
            _ => Err(format!("invalid opcode {}", op)),
        }
    }
}

pub struct IntcodeVM {
    pub pc: usize,
    pub mem: Vec<i64>,
    pub input_queue: VecDeque<i64>,
    pub relbase: i64,
}

impl IntcodeVM {
    pub fn with_mem(mem: &ProgMem) -> Self {
        Self {
            pc: 0,
            mem: mem.0.clone(),
            input_queue: VecDeque::new(),
            relbase: 0,
        }
    }

    pub fn step<FIN, FOUT>(&mut self, input: &mut FIN, output: &mut FOUT) -> StepResult
        where FIN: FnMut() -> Option<i64>, FOUT: FnMut(i64)
    {
        if self.pc >= self.mem.len() {
            return StepResult::InvalidInstr(format!("pc {} greater than max mem {}", self.pc, self.mem.len()));
        }
        let instr = self.mem[self.pc];

        let op: Result<Opcode, _> = instr.try_into();
        if let Err(msg) = op {
            return StepResult::InvalidInstr(msg);
        }
        let op = op.unwrap();
        if self.pc + op.size() > self.mem.len() {
            self.mem.resize(self.pc + op.size(), 0);
        }

        let mut args = self.mem[self.pc + 1 .. self.pc + op.size()].to_owned();
        for idx in 0..args.len() {
            let mode = (instr / 10i64.pow(idx as u32 + 2)) % 10;
            match mode {
                0 |    // position
                2 => { // relative
                    let addr = args[idx] + if mode == 0 { 0 } else { self.relbase };
                    if addr < 0 {
                        return StepResult::InvalidInstr(
                            format!("negative argument: {}", addr)
                        );
                    }
                    if addr as usize >= self.mem.len() {
                        self.mem.resize(addr as usize + 1, 0);
                    }
                    if !op.stores_to(idx) {
                        args[idx] = self.mem[addr as usize];
                    }
                    else {
                        args[idx] = addr;
                    }
                },
                1 => { // immediate
                    if op.stores_to(idx) {
                        return StepResult::InvalidInstr("invalid address mode for destination argument".into());
                    }
                },
                _ => { return StepResult::InvalidInstr(format!("invalid address mode {mode}")); }
            }
        }

        match op {
            Opcode::Add => { self.mem[args[2] as usize] = args[0] + args[1]; },
            Opcode::Mul => { self.mem[args[2] as usize] = args[0] * args[1]; },
            Opcode::Inp => {
                if let Some(val) = self.input_queue.pop_front() {
                    self.mem[args[0] as usize] = val;
                }
                else if let Some(val) = input() {
                    self.mem[args[0] as usize] = val;
                }
                else { return StepResult::InputNeeded; }
            },
            Opcode::Out => { output(args[0]) },
            Opcode::Jnz => { if args[0] != 0 { return self.do_jump(args[1]); }},
            Opcode::Jz =>  { if args[0] == 0 { return self.do_jump(args[1]); }},
            Opcode::Lt =>  { self.mem[args[2] as usize] = if args[0] < args[1] {1} else {0} },
            Opcode::Eq =>  { self.mem[args[2] as usize] = if args[0] == args[1] {1} else {0} },
            Opcode::Rlb => { self.relbase += args[0]; },
            Opcode::Hlt => { return StepResult::Halt; },
        }
        self.pc += op.size();
        StepResult::Ok
    }

    fn do_jump(&mut self, addr: i64) -> StepResult {
        if !(0..self.mem.len() as i64).contains(&addr) {
            return StepResult::InvalidInstr(format!("jump destination {addr} out of range"));
        }
        self.pc = addr as usize;
        StepResult::Ok
    }

    pub fn run(&mut self) -> Result<(), RunErr> {
        let mut input = || None;
        let mut output = |v| { println!("{v}"); };
        loop {
            match self.step(&mut input, &mut output) {
                StepResult::Ok => continue,
                StepResult::Halt => return Ok(()),
                StepResult::InputNeeded => return Err(RunErr::InputNeeded),
                StepResult::InvalidInstr(err) => return Err(RunErr::InvalidInstr(err)),
            }
        }
    }

    pub fn run_with_cb<F1,F2>(&mut self, input: &mut F1, output: &mut F2) -> Result<(), RunErr>
        where F1: FnMut() -> Option<i64>, F2: FnMut(i64)
    {
        loop {
            match self.step(input, output) {
                StepResult::Ok => continue,
                StepResult::Halt => return Ok(()),
                StepResult::InputNeeded => return Err(RunErr::InputNeeded),
                StepResult::InvalidInstr(err) => return Err(RunErr::InvalidInstr(err)),
            }
        }
    }
}
