use std::io;
use std::io::prelude::*;
use std::collections::HashSet;
use std::ops::Add;
use std::cmp::PartialEq;
use regex::Regex;

#[derive(Clone, PartialEq)]
enum Op { Nop, Jmp, Acc }

#[derive(Clone)]
struct Instruction { op: Op, arg: i32 }

struct State { acc: i32, ip: i32 }

impl Add<&Instruction> for &State {
    type Output = State;

    fn add(self, other: &Instruction) -> Self::Output {
        match other.op {
            Op::Acc => State { acc: self.acc + other.arg, ip: self.ip + 1 },
            Op::Jmp => State { acc: self.acc, ip: self.ip + other.arg },
            Op::Nop => State { acc: self.acc, ip: self.ip + 1 }
        }
    }
}

fn main() {
    let r = Regex::new(r"^([a-z]+) ((\+|\-)\d+)$").unwrap();
    let input : Vec<Instruction> = 
        io::stdin().lock().lines()
                   .map(|l| l.unwrap())
                   .take_while(|l| l != "")
                   .map(|l| {
                        let c = r.captures(&l).unwrap();
                        Instruction {
                            op: match &c[1] {
                                "acc" => Op::Acc,
                                "jmp" => Op::Jmp,
                                _ => Op::Nop
                            },
                            arg: c[2].parse::<i32>().unwrap()   
                        }
                   })
                   .collect();

    let mut visited : HashSet<i32> = HashSet::new();
    let mut state = State { acc: 0, ip: 0 };

    let answer_one = loop {
        if !visited.insert(state.ip) {
            break state.acc;
        }

        state = &state + &input[state.ip as usize];
    };

    println!("Part One: {}", answer_one);

    let answer_two = input.iter()
        .enumerate()
        .filter_map(|(ip, i)| if i.op != Op::Acc { Some(ip as i32) } else { None })
        .find_map(|ip| {
            let mut input = input.clone();
            input[ip as usize].op = if input[ip as usize].op == Op::Nop { Op::Jmp } else { Op::Nop };

            let mut visited : HashSet<i32> = HashSet::new();
            let mut state = State { acc: 0, ip: 0 };

            loop {
                if !visited.insert(state.ip) {
                    break None;
                }

                if state.ip >= input.len() as i32 {
                    break Some(state.acc);
                }
        
                state = &state + &input[state.ip as usize];
            }
        }).unwrap();

    println!("Part Two: {}", answer_two);
}