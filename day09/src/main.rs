use std::io;
use std::io::prelude::*;

fn main() {
    let input : Vec<u64> = 
        io::stdin().lock().lines()
                   .map(|l| l.unwrap())
                   .take_while(|l| l != "")
                   .map(|l| l.parse::<u64>().unwrap())
                   .collect();

    let (i, answer_one) = input.iter()
        .enumerate()
        .skip(25)
        .zip(input.windows(25))
        .find_map(|((i, n), w)| {
            w.iter()
                .enumerate()
                .find_map(|(i, x)| w.iter().skip(i+1).find(|y| x + *y == *n))
                .map_or_else(|| Some((i, n)), |_| None)
        })
        .unwrap();

    println!("Part One: {}", answer_one);

    let answer_two = (0..i)
        .find_map(|from| {
            (from + 1..i)
                .map(|to| (from, to, input.iter().skip(from).take(to-from).sum::<u64>()))
                .find(|(_, _, s)| s == answer_one)
        })
        .map(|(from, to, _)| input.iter().skip(from).take(to-from).min().unwrap() + input.iter().skip(from).take(to-from).max().unwrap())
        .unwrap();

    println!("Part Two: {}", answer_two);
}