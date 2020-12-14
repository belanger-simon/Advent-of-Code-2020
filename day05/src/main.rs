use std::io;
use std::io::prelude::*;

fn main() {
    let mut input : Vec<u32> = 
        io::stdin().lock().lines()
                   .map(|l| l.unwrap())
                   .take_while(|l| l != "")
                   .map(|id| {
                        id.chars().take(7)
                            .fold((0, 127), |(min, max), c| if c == 'F' { (min, max - (max - min) / 2 - 1) } else { (min + (max - min) / 2 + 1, max) }).0 * 8 +
                        id.chars().skip(7)
                            .fold((0, 7), |(min, max), c| if c == 'L' { (min, max - (max - min) / 2 - 1) } else { (min + (max - min) / 2 + 1, max) }).0
                   })
                   .collect();

    input.sort();

    let answer_one = input.last().unwrap();

    println!("Part One: {}", answer_one);

    let answer_two = input.iter()
        .skip(1)
        .zip(input.iter())
        .filter(|(l, r)| *l - *r == 2)
        .map(|(x, _)| x - 1)
        .next().unwrap();


    println!("Part Two: {}", answer_two);
}
