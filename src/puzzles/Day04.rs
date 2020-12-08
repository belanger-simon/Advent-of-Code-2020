use crate::puzzles::Puzzle;
use std::collections::HashMap;
use regex::Regex;

struct Day04 { }

pub fn create() -> impl Puzzle {
    Day04 { }
}

impl Puzzle for Day04 {
    fn solve_part_one(&self, input: &Vec<String>) -> Option<String> {
        let input = parse_input(&input);

        Some(
            input.iter()
                 .filter(|p| p.len() == 7)
                 .count().to_string())
    }

    fn solve_part_two(&self, input: &Vec<String>) -> Option<String> {
        let input = parse_input(input);
        let mut validators = HashMap::new();

        validators.insert(String::from("byr"), Regex::new(r"^(19[2-9][0-9]|200[0-2])$").unwrap());
        validators.insert(String::from("iyr"), Regex::new(r"^(201[0-9]|2020)$").unwrap());
        validators.insert(String::from("eyr"), Regex::new(r"^(202[0-9]|2030)$").unwrap());
        validators.insert(String::from("hgt"), Regex::new(r"^(((1[5-8][0-9]|19[0-3])cm)|((59|6[0-9]|7[0-6])in))$").unwrap());
        validators.insert(String::from("hcl"), Regex::new(r"^(#[0-9a-z]{6})$").unwrap());
        validators.insert(String::from("ecl"), Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap());
        validators.insert(String::from("pid"), Regex::new(r"^(\d{9})$").unwrap());
        
        Some(
            input.into_iter()
                 .filter(|p| p.len() == 7 && p.iter().all(|(f, v)| validators.get(f).map(|r| r.is_match(v)).unwrap()))
                 .count().to_string())
    }
}

type Passport = HashMap<String, String>;


fn parse_input(input: &[String]) -> Vec<Passport> {
    let r = Regex::new(r"([a-z]{3}):([^ ]+) ?").unwrap();

    input.split(|x| x.len() == 0)
         .map(|passport| 
            passport.iter()
                    .map(|l| r.captures_iter(&l).map(|c| (c[1].to_owned(), c[2].to_owned())))
                    .flatten()
                    .filter(|f| f.0 != "cid")
                    .collect::<Passport>()
         )
         .collect()
}