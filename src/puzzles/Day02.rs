use crate::puzzles::Puzzle;
use regex::Regex;

struct Day02 { }

pub fn create() -> impl Puzzle {
    Day02 { }
}

impl Puzzle for Day02 {
    fn solve_part_one(&self, input: &Vec<String>) -> Option<String> {
        let input = parse_input(&input);

        Some(input.iter()
                  .filter(|i| {
                      between(&i.policy.min, &i.policy.max, &i.password.chars().filter(|c| c == &i.policy.character).count())
                  })
                  .count().to_string())
    }

    fn solve_part_two(&self, input: &Vec<String>) -> Option<String> {
        let input = parse_input(input);

        Some(input.iter()
                  .filter(|i| {
                      i.password.chars().nth(i.policy.min - 1).map(|c| c == i.policy.character).unwrap_or(false) ^
                      i.password.chars().nth(i.policy.max - 1).map(|c| c == i.policy.character).unwrap_or(false)
                  })
                  .count().to_string())
    }
}

fn parse_input(input: &[String]) -> Vec<Input> {
    let r = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();

    input.iter()
        .map(|i| r.captures(&i).map(|c| {
            Input { 
                policy: PasswordPolicy {
                    min: c[1].parse::<usize>().unwrap(),
                    max: c[2].parse::<usize>().unwrap(),
                    character: c[3].chars().next().unwrap()
                },
                password: c[4].to_owned()
            } 
        }).unwrap())
        .collect::<Vec<_>>()
}

fn between<T: std::cmp::PartialOrd>(min: &T, max: &T, value: &T) -> bool {
    min <= value && value <= max
}

struct Input {
    policy: PasswordPolicy,
    password: String
}

struct PasswordPolicy {
    min: usize,
    max: usize,
    character: char
}