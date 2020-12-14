use std::io;
use std::io::prelude::*;
use std::ops::{Add, Mul};
use std::collections::HashSet;

#[derive(Hash, Clone, Copy, Eq, PartialEq, Debug)]
struct Position {
    x: i32,
    y: i32
}

impl Add<&Position> for Position {
    type Output = Position;

    fn add(self, other: &Position) -> Position {
        Position { x: self.x + other.x, y: self.y + other.y }
    }
}

impl Mul<&i32> for &Position {
    type Output = Position;

    fn mul(self, other: &i32) -> Position {
        Position { x: self.x * other, y: self.y * other }
    }
}

fn main() {
    let input : HashSet<Position> = 
        io::stdin().lock().lines()
                   .map(|l| l.unwrap())
                   .take_while(|l| l != "")
                   .collect::<Vec<_>>().iter()
                   .zip(0i32..)
                   .flat_map(|(row, y)| row.chars().zip(0i32..).filter_map(move |(c, x)| if c == 'L' { Some(Position { x, y }) } else { None }))
                   .collect();
    
    let neighbours : Vec<Position> = (-1..=1)
        .flat_map(|x| (-1..=1).map(move |y| Position { x, y }))
        .filter(|p| p.x != 0 || p.y != 0)
        .collect();

    let max_x = input.iter().map(|p| p.x).max().unwrap();
    let max_y = input.iter().map(|p| p.y).max().unwrap();

    let mut occupied : HashSet<&Position> = HashSet::new();

    let answer_one = loop {
        let o : HashSet<&Position> = 
            input.iter()
                .filter(|p| {
                    let n = neighbours.iter().filter(|n| occupied.contains(&(**p + *n))).take(4).count();
                    occupied.contains(p) && n < 4 || n == 0
                })
                .collect();

        if o == occupied {
            break occupied.len();
        }

        occupied = o;
    };

    println!("Part One: {}", answer_one);

    let mut occupied : HashSet<&Position> = HashSet::new();

    let answer_two = loop {
        let o : HashSet<&Position> = 
            input.iter()
                .filter(|p| {
                    let n = neighbours.iter()
                        .filter(|n| {
                            (1i32..)
                                .map(|d| **p + &(*n * &d))
                                .take_while(|n| n.x >= 0 && n.x <= max_x && n.y >= 0 && n.y <= max_y) 
                                .find(|n| input.contains(n))
                                .map(|n| occupied.contains(&n))
                                .unwrap_or(false)
                        })
                        .count();

                    occupied.contains(p) && n < 5 || n == 0
                })
                .collect();

        if o == occupied {
            break occupied.len();
        }

        occupied = o;
    };

    println!("Part Two: {}", answer_two);
}