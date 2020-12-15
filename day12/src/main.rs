use std::io;
use std::io::prelude::*;

fn main() {
    let input : Vec<_> = io::stdin().lock().lines()
        .map(|l| l.unwrap())
        .take_while(|l| l != "")
        .map(|l| { 
            let (a, v) = l.split_at(1);

            (a.chars().next().unwrap(), v.parse::<i32>().unwrap())
        })
        .collect();

    let answer_one = input.iter()
        .fold(((0, 0), (1, 0)), |((x, y), (wx, wy)), (a, v)| {
            match a {
                'L' => ((x, y), (0..v / 90).fold((wx, wy), |(wx, wy), _| (-wy, wx))),
                'R' => ((x, y), (0..v / 90).fold((wx, wy), |(wx, wy), _| (wy, -wx))),
                'F' => ((x + wx * v, y + wy * v), (wx, wy)),
                'E' => ((x + v, y), (wx, wy)),
                'N' => ((x, y + v), (wx, wy)),
                'W' => ((x - v, y), (wx, wy)),
                'S' => ((x, y - v), (wx, wy)),
                _ => ((x, y), (wx, wy))
            }
        });

    println!("Part One: {}", answer_one.0.0.abs() + answer_one.0.1.abs());

    let answer_two = input.iter()
        .fold(((0, 0), (10, 1)), |((x, y), (wx, wy)), (a, v)| {
            match a {
                'L' => ((x, y), (0..v / 90).fold((wx, wy), |(wx, wy), _| (-wy, wx))),
                'R' => ((x, y), (0..v / 90).fold((wx, wy), |(wx, wy), _| (wy, -wx))),
                'F' => ((x + wx * v, y + wy * v), (wx, wy)),
                'E' => ((x, y), (wx + v, wy)),
                'N' => ((x, y), (wx, wy + v)),
                'W' => ((x, y), (wx - v, wy)),
                'S' => ((x, y), (wx, wy - v)),
                _ => ((x, y), (wx, wy))
            }
        });

    println!("Part Two: {}", answer_two.0.0.abs() + answer_two.0.1.abs());
}