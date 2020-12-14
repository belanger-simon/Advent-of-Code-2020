use std::io;
use std::io::prelude::*;

fn main() {
    let mut input : Vec<u32> = 
        io::stdin().lock().lines()
                   .map(|l| l.unwrap())
                   .take_while(|l| l != "")
                   .map(|l| l.parse::<u32>().unwrap())
                   .collect();

    input.push(0);
    input.sort();
    input.push(input.last().unwrap() + 3);

    let answer_one = input.iter()
        .zip(input.iter().skip(1))
        .fold((0, 0), |(o, t), (l, n)| if n - l == 1 { (o + 1, t) } else { (o, t + 1) });

    println!("Part One: {}", answer_one.0 * answer_one.1);

    let answer_two = input.iter()
        .skip(1)
        .zip(input.iter())
        .fold((0, 1, 0, 0), |c, (x, last)| {
            if x - last == 3 {
                match c.0 {
                    3 => (1, c.1 + 1, c.2, c.3),
                    4 => (1, c.1, c.2 + 1, c.3),
                    5 => (1, c.1, c.2, c.3 + 1),
                    _ => (1, c.1, c.2, c.3)
                }
            } else {
                (c.0 + 1, c.1, c.2, c.3)
            } 
        });

    println!("Part Two: {}", 2i64.pow(answer_two.1) * 4i64.pow(answer_two.2) * 7i64.pow(answer_two.3));
}