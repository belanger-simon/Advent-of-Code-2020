use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
use regex::Regex;

enum Instruction {
    Mask(String),
    Mem(u64, u64)
}

fn main() {
    let mask_r = Regex::new(r"^mask = ([10X]{36})$").unwrap();
    let mem_r = Regex::new(r"mem\[(\d+)\] = (\d+)$").unwrap();

    let input : Vec<_> = io::stdin().lock()
        .lines()
        .map(|l| l.unwrap())
        .take_while(|l| l != "")
        .map(|l| {
            mask_r
                .captures(&l[..])
                .map_or_else(
                    || mem_r.captures(&l[..]).map(|c| Instruction::Mem(c[1].parse().unwrap(), c[2].parse().unwrap())).unwrap(),
                    |c| Instruction::Mask(c[1].to_owned())
                )
        })
        .collect::<Vec<_>>();
    
    let bit_flags = (0..=35).map(|b| 1u64 << b).rev().collect::<Vec<_>>();

    let answer_one = input.iter()
        .fold((HashMap::new(), ""), |(mut mem, mask), inst| {
            match inst { 
                Instruction::Mask(mask) => (mem, mask), 
                Instruction::Mem(addr, val) => { 
                    mem.insert(addr, mask.chars().zip(bit_flags.iter()).filter(|(m, _)| m != &'X').fold(*val, |val, (m, b)| if m == '1' { val | b } else { val & !b }));
                    (mem, mask)
                }
            }
        });

    println!("Part One: {}", answer_one.0.values().map(|v| *v).sum::<u64>());

    let answer_two = input.iter()
    .fold((HashMap::new(), ""), |(mut mem, mask), inst| {
        match inst { 
            Instruction::Mask(mask) => (mem, mask), 
            Instruction::Mem(addr, val) => { 
                let addr = mask.chars().zip(bit_flags.iter()).filter_map(|(m, b)| if m == '1' { Some(b) } else { None }).fold(*addr, |addr, b| addr | b);
                mask.chars()
                    .zip(bit_flags.iter())
                    .filter_map(|(m, b)| if m == 'X' { Some(b) } else { None })
                    .fold(vec![addr], |addrs, b| addrs.iter().flat_map(|addr| vec![addr | b, addr & !b]).collect())
                    .iter().for_each(|&addr| { mem.insert(addr, val); });

                (mem, mask)
            }
        }
    });

    println!("Part Two: {}", answer_two.0.values().map(|v| *v).sum::<u64>());
}