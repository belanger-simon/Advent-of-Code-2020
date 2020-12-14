use std::io;
use std::io::prelude::*;

fn main() {
    let mut input : Vec<_> = 
        io::stdin().lock().lines()
                   .map(|l| l.unwrap())
                   .take_while(|l| l != "")
                   .map(|l| l.parse::<i64>().unwrap())
                   .collect();

    input.sort();

    let answer_one = 
        input.iter().enumerate()
             .map(|(i, x)| (i, x, 2020 - x))
             .find_map(|(i, x, s)| {
                 input.iter()
                      .skip(i + 1)
                      .take_while(|y| y <= &&s)
                      .last()
                      .and_then(|y| if y == &s { Some(x * y) } else { None })
             }).unwrap();

    println!("Part One: {}", answer_one);

    let answer_two = 
        input.iter().enumerate()
             .map(|(i, x)| (i, x, 2020 - x))
             .find_map(|(i, x, s)| {
                 input.iter().enumerate()
                      .skip(i + 1)
                      .map(|(j, y)| (j, y, s - y))
                      .find_map(|(j, y, s)| {
                          input.iter()
                               .skip(j + 1)
                               .take_while(|z| z <= &&s)
                               .last()
                               .and_then(|z| if z == &s { Some(x * y * z) } else { None })
                      })
             }).unwrap();

    println!("Part Two: {}", answer_two);
}
