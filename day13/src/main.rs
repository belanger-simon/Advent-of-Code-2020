use std::io;
use std::io::prelude::*;

fn main() {
    let input : Vec<_> = io::stdin().lock().lines()
        .map(|l| l.unwrap())
        .take(2)
        .collect();
    let input : (u32, Vec<&str>) = (
        input[0].parse::<u32>().unwrap(),
        input[1].split(',').collect()
    );

    let answer_one = input.1.iter()
        .filter_map(|id| match id { &"x" => None, t @ _ => Some(t.parse::<u32>().unwrap()) })
        .map(|id| (id, (id - (input.0 % id)) % id))
        .min_by_key(|(_, t)| *t)
        .unwrap();

    println!("Part One: {}", answer_one.0 * answer_one.1);

    // A sort of LCD problem. Align the buses one by one. Once they are co-aligned at a time T, they will always be
    // Aligned at every time-step they were found to be aligned.
    let answer_two = input.1.iter()
        .zip(0..)
        .filter_map(|(id, offset)| if id != &"x" { Some((id.parse::<u64>().unwrap(), offset)) } else { None })
        .fold((0u64, 1u64), |(time, step), (id, offset)| {
            (time..u64::MAX)
                .step_by(step as usize)
                .find(|t| (t + offset) % id == 0)
                .map(|time| (time, step * id))
                .unwrap()
        });

    println!("Part Two: {}", answer_two.0);
}