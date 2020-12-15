use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
    let input : Vec<_> = io::stdin().lock()
        .lines().next().unwrap().unwrap()
        .split(',').map(|n| n.parse::<u32>().unwrap()).collect();
    
    let answer_one : (HashMap<u32, u32>, u32) = (input.len() as u32 + 1..=2020)
        .fold((input.iter().take(input.len() - 1).zip(1..).map(|(n, t)| (*n, t)).collect(), *input.last().unwrap()), |(mut s, ln), t|
            match s.insert(ln, t - 1) {
                Some(tb) => (s, t - tb - 1),
                _ => (s, 0)
            }
        );

    println!("Part One: {}", answer_one.1);

    let answer_two : (HashMap<u32, u32>, u32) = (input.len() as u32 + 1..=30000000)
        .fold((input.iter().take(input.len() - 1).zip(1..).map(|(n, t)| (*n, t)).collect(), *input.last().unwrap()), |(mut s, ln), t|
            match s.insert(ln, t - 1) {
                Some(tb) => (s, t - tb - 1),
                _ => (s, 0)
            }
        );

    println!("Part Two: {}", answer_two.1);
}