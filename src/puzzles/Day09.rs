use crate::puzzles::Puzzle;
use std::collections::HashMap;
use std::collections::VecDeque;

struct Day09 { }

pub fn create() -> impl Puzzle {
    Day09 { }
}

impl Puzzle for Day09 {
    fn solve_part_one(&self, input: &Vec<String>) -> Option<String> {
        let data = parse_input(input);
        let preamble_size = if data.len() < 25 { 5 } else { 25 };
        let mut preamble = Preamble::new(preamble_size);

        data.iter()
            .filter(|v| !preamble.push(**v))
            .map(|v| v.to_string())
            .next()
    }

    fn solve_part_two(&self, input: &Vec<String>) -> Option<String> {
        let data = parse_input(input);
        let preamble_size = if data.len() < 25 { 5 } else { 25 };
        let mut preamble = Preamble::new(preamble_size);

        let target = &data.iter()
                          .filter(|v| !preamble.push(**v))
                          .next()
                          .unwrap();

        for (from, x) in data.iter().enumerate() {
            let mut sum = *x;

            if sum >= **target {
                return None;
            }

            for (to, y) in data.iter().enumerate().skip(from + 1) {
                sum += y;

                if sum == **target {
                    let min = data.iter().skip(from).take(to-from).min().unwrap();
                    let max = data.iter().skip(from).take(to-from).max().unwrap();

                    return Some((min + max).to_string());
                }
                else if sum > **target {
                    break;
                }
            }
        }

        None
    }
}

fn parse_input(input: &[String]) -> Vec<i64> {
    input.iter()
        .map(|o| o.parse::<i64>().unwrap())
        .collect()
}

struct Preamble {
    size: usize,
    numbers: VecDeque<i64>,
    valid: HashMap<i64, i32>
}

impl Preamble {
    fn new(size: usize) -> Preamble {
        Preamble {
            size,
            numbers: VecDeque::new(),
            valid: HashMap::new()
        }
    }

    fn push(&mut self, value: i64) -> bool {
        if self.numbers.len() < self.size {
            self.add_last(value);

            true
        }
        else if self.is_valid(&value) {
            self.remove_first();
            self.add_last(value);

            true
        }
        else {
            false
        }
    }

    fn remove_first(&mut self) {
        let value = self.numbers.pop_front().unwrap();
        
        for x in self.numbers.iter() {
            self.valid.entry(x + value).and_modify(|e| *e -= 1);
        }
    }

    fn add_last(&mut self, value: i64) {
        if !self.numbers.is_empty() {
            for x in self.numbers.iter() {
                self.valid.entry(x + value).and_modify(|e| *e += 1).or_insert(1);
            }
        }

        self.numbers.push_back(value);
    }

    fn is_valid(&self, value: &i64) -> bool {
        match self.valid.get(value) {
            Some(c) => c > &0,
            None => false
        }
    }
}