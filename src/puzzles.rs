pub trait Puzzle {
    fn solve_part_one(&self, _input: &Vec<String>) -> Option<String> {
        None
    }

    fn solve_part_two(&self, _input: &Vec<String>) -> Option<String> {
        None
    }
}

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;

pub fn get_puzzle(day: u8) -> Option<Box<dyn Puzzle>> {
    match day {
        1 => Some(Box::new(day01::create())),
        2 => Some(Box::new(day02::create())),
        3 => Some(Box::new(day03::create())),
        4 => Some(Box::new(day04::create())),
        5 => Some(Box::new(day05::create())),
        _ => None
    }
}