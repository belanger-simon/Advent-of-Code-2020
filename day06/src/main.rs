use std::io;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() {
    let input : Vec<Vec<HashSet<_>>> = 
        io::stdin().lock().lines()
                   .map(|l| l.unwrap())
                   .take_while(|l| l != ".")
                   .collect::<Vec<_>>()
                   .split(|l| l.len() == 0)
                   .map(|grp| grp.iter().map(|q| q.chars().collect()).collect())
                   .collect();

    let answer_one = input.iter()
        .map(|grp| grp.iter().fold(HashSet::new(), |l, r| l.union(r).cloned().collect()).len())
        .sum::<usize>();

    println!("Part One: {}", answer_one);

    let answer_two = input.iter()
        .map(|grp| grp.iter().fold(('a'..='z').collect::<HashSet<_>>(), |l, r| l.intersection(r).cloned().collect()).len())
        .sum::<usize>();

    println!("Part Two: {}", answer_two);
}
