pub trait Puzzle {
    fn solve_part_one(&self, _input: &Vec<String>) -> Option<String> {
        None
    }

    fn solve_part_two(&self, _input: &Vec<String>) -> Option<String> {
        None
    }
}

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;


pub fn get_puzzle(day: u8) -> Option<Box<dyn Puzzle>> {
    match day {
        1 => Some(Box::new(day01::create())),
        2 => Some(Box::new(day02::create())),
        3 => Some(Box::new(day03::create())),
        4 => Some(Box::new(day04::create())),
        5 => Some(Box::new(day05::create())),
        6 => Some(Box::new(day06::create())),
        7 => Some(Box::new(day07::create())),
        8 => Some(Box::new(day08::create())),
        9 => Some(Box::new(day09::create())),
        10 => Some(Box::new(day10::create())),
        _ => None
    }
}