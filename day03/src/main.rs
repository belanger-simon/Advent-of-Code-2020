use std::io;
use std::io::prelude::*;
use std::ops::Mul;

#[derive(Copy, Clone, Debug)]
struct Position {
    x: usize,
    y: usize
}

impl Mul<usize> for Position {
    type Output = Position;

    fn mul(self, rhs: usize) -> Self::Output {
        Position { x: self.x * rhs, y: self.y * rhs }
    }
}

const TREE: char = '#';

fn main() {
    let input : Vec<Vec<_>> = 
        io::stdin().lock().lines()
                   .map(|l| l.unwrap())
                   .take_while(|l| l != "")
                   .map(|row| row.chars().collect())
                   .collect();

    let height = input.len();
    let width = input[0].len();

    let slope = Position { x: 3, y: 1 };
    let answer_one = 
        (0..)
            .map(|d| slope * d)
            .take_while(|p| p.y < height)
            .filter(|p| input[p.y][p.x % width] == '#')
            .count();

    println!("Part One: {}", answer_one);

    let slopes = [
        Position { x: 1, y: 1 },
        Position { x: 5, y: 1 },
        Position { x: 7, y: 1 },
        Position { x: 1, y: 2 }
    ];

    let answer_two = 
        slopes.iter()
            .map(|slope| {
                (0..)
                    .map(|d| *slope * d)
                    .take_while(|p| p.y < height)
                    .filter(|p| input[p.y][p.x % width] == TREE)
                    .count() as u32
            })
            .product::<u32>() * answer_one as u32;

    println!("Part Two: {}", answer_two);
}
