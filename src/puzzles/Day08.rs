use crate::puzzles::Puzzle;
use std::iter::FromIterator;
use std::str::FromStr;
use std::collections::HashSet;
use regex::Regex;

struct Day08 { }

pub fn create() -> impl Puzzle {
    Day08 { }
}

impl Puzzle for Day08 {
    fn solve_part_one(&self, input: &Vec<String>) -> Option<String> {
        let program = parse_input(input);
        let mut state = ProgramState::new();
        let mut visited_ip : HashSet<i32> = HashSet::new();

        while visited_ip.insert(state.ip) {
            match program.step(&state) {
                StepResult::Ok(next) => state = next,
                _ => return None
            };
        }

        Some(state.acc.to_string())
    }

    fn solve_part_two(&self, input: &Vec<String>) -> Option<String> {
        let mut program = parse_input(input);
        let mut to_test : Vec<i32> = program.instructions.iter()
                                            .enumerate()
                                            .filter_map(|(ip, i)| {
                                                match i.0 {
                                                    Operation::Nop | Operation::Jmp => Some(ip as i32),
                                                    _ => None
                                                }
                                            })
                                            .collect();

        while let Some(test_ip) = to_test.pop() {
            let old_i = program.instructions[test_ip as usize];
            let new_op = match old_i.0 {
                Operation::Jmp => Operation::Nop,
                _ => Operation::Jmp 
            };

            program.instructions[test_ip as usize] = Instruction(new_op, old_i.1);

            let mut state = ProgramState::new();
            let mut visited_ip : HashSet<i32> = HashSet::new();

            while visited_ip.insert(state.ip) {
                match program.step(&state) {
                    StepResult::Ok(next) => state = next,
                    StepResult::Terminated(acc) => return Some(acc.to_string()),
                    _ => return None
                };
            }

            program.instructions[test_ip as usize] = old_i;
        }

        None
    }
}

fn parse_input(input: &[String]) -> Program {
    input.iter()
        .map(|o| o.parse::<Instruction>().unwrap())
        .collect()
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Acc,
    Jmp,
    Nop
}

#[derive(Debug, Clone, Copy)]
struct Instruction(Operation, i32);

impl FromStr for Instruction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = Regex::new(r"^([a-z]+) (\+|\-)(\d+)$").unwrap();
        
        match r.captures(s) {
            Some(c) => {
                let mut arg = c[3].parse::<i32>().unwrap();

                if &c[2] == "-" {
                    arg = arg * -1;
                }
        
                match &c[1] {
                    "nop" => Ok(Instruction(Operation::Nop, arg)),
                    "acc" => Ok(Instruction(Operation::Acc, arg)),
                    "jmp" => Ok(Instruction(Operation::Jmp, arg)),
                    _ => Err(())
                }
            },
            _ => Err(())
        }
        
    }
}

struct Program {
    instructions: Vec<Instruction>
}

impl Program {
    fn new(instructions: Vec<Instruction>) -> Self {
        Program {
            instructions
        }
    }
}

impl FromIterator<Instruction> for Program {
    fn from_iter<I: IntoIterator<Item=Instruction>>(iter: I) -> Self {
        Program::new(iter.into_iter().collect())
    }
}

struct ProgramState {
    acc: i32,
    ip: i32
}

enum StepResult {
    Ok(ProgramState),
    Err,
    Terminated(i32)
}

impl ProgramState {
    fn new() -> ProgramState {
        ProgramState { 
            acc: 0,
            ip: 0
        }
    }
}

impl Program {
    fn step(&self, state: &ProgramState) -> StepResult {
        match state.ip {
            ip if ip < 0 => StepResult::Err,
            ip if ip as usize >= self.instructions.len()=> StepResult::Terminated(state.acc),
            ip => {
                match self.instructions[ip as usize] {
                    Instruction(Operation::Acc, arg) => 
                        StepResult::Ok(ProgramState { ip: state.ip + 1, acc: state.acc + arg }),
                    Instruction(Operation::Jmp, arg) => 
                        StepResult::Ok(ProgramState { ip: state.ip + arg, ..*state }),
                    Instruction(Operation::Nop, _) => 
                        StepResult::Ok(ProgramState { ip: state.ip + 1, ..*state })
                }
            }
        }
    }
}

