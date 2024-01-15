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

#[derive(Clone)]
pub struct IntcodeVM {
    pub pc: usize,
    pub mem: Vec<i64>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum StepResult {
    Ok,
    Halt,
    InvalidInstr,
    InvalidPC,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RunErr {
    InvalidInstr,
    InvalidPC,
}

enum Operand {
    Add,
    Mul,
}
impl Operand {
    fn size(&self) -> usize {
        match self {
            Operand::Add => 4,
            Operand::Mul => 4,
            //Operand::Halt => 1,
        }
    }
}

impl IntcodeVM {
    pub fn with_mem(mem: &ProgMem) -> Self {
        Self { pc: 0, mem: mem.0.clone() }
    }

    pub fn step(&mut self) -> StepResult {
        if self.pc >= self.mem.len() {
            return StepResult::InvalidInstr;
        }
        match self.mem[self.pc] {
            1 => self.instr(Operand::Add),
            2 => self.instr(Operand::Mul),
            99 => StepResult::Halt,
            _ => StepResult::InvalidInstr,
        }
    }

    pub fn run(&mut self) -> Result<(), RunErr> {
        loop {
            match self.step() {
                StepResult::Ok => continue,
                StepResult::Halt => return Ok(()),
                StepResult::InvalidInstr => return Err(RunErr::InvalidInstr),
                StepResult::InvalidPC => return Err(RunErr::InvalidPC),
            }
        }
    }

    fn instr(&mut self, op: Operand) -> StepResult {
        if self.pc + op.size() >= self.mem.len() {
            return StepResult::InvalidPC;
        }
        let args = self.mem[self.pc + 1 .. self.pc + op.size()].to_owned();
        let addr0 = if op.size() >= 2 && (0..self.mem.len() as i64).contains(&args[0]) {
            Some(args[0] as usize)
        } else { None };
        let addr1 = if op.size() >= 3 && (0..self.mem.len() as i64).contains(&args[1]) {
            Some(args[1] as usize)
        } else { None };
        let addr2 = if op.size() >= 4 && (0..self.mem.len() as i64).contains(&args[2]) {
            Some(args[2] as usize)
        } else { None };

        match op {
            Operand::Add if addr0.is_some() && addr1.is_some() && addr2.is_some() => {
                self.mem[addr2.unwrap()] = self.mem[addr0.unwrap()] + self.mem[addr1.unwrap()];
            },
            Operand::Mul if addr2.is_some() => {
                self.mem[addr2.unwrap()] = self.mem[addr0.unwrap()] * self.mem[addr1.unwrap()];
            },
            _ => { return StepResult::InvalidInstr; },
        }
        self.pc += op.size();
        StepResult::Ok
    }
}
