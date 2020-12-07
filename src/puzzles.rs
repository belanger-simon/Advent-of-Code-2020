pub trait Puzzle {
    fn solve_part_one(&self, input: &Vec<String>) -> Option<String> {
        None
    }

    fn solve_part_two(&self, input: &Vec<String>) -> Option<String> {
        None
    }
}

pub mod day01;

pub fn get_puzzle(day: u8) -> Option<impl Puzzle> {
    match day {
        1 => Some(day01::create()),
        _ => None
    }
}