use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
use regex::Regex;

fn main() {
    let r = Regex::new(r"([a-z]{3}):([^ ]+) ?").unwrap();
    let input : Vec<HashMap<_,_>> = 
        io::stdin().lock().lines()
                   .map(|l| l.unwrap())
                   .take_while(|l| l != ".")
                   .collect::<Vec<String>>()
                   .as_slice().split(|x| x.len() == 0)
                   .map(|p| {
                       p.iter()
                        .flat_map(|l| r.captures_iter(&l).map(|c| (c[1].to_owned(), c[2].to_owned())))
                        .filter(|f| f.0 != "cid")
                        .collect()
                   })
                   .collect();

    let answer_one = input.iter().filter(|p| p.len() == 7).count();

    println!("Part One: {}", answer_one);

    let rules : HashMap<String, Regex> = vec![
        (String::from("byr"), Regex::new(r"^(19[2-9][0-9]|200[0-2])$").unwrap()),
        (String::from("iyr"), Regex::new(r"^(201[0-9]|2020)$").unwrap()),
        (String::from("eyr"), Regex::new(r"^(202[0-9]|2030)$").unwrap()),
        (String::from("hgt"), Regex::new(r"^(((1[5-8][0-9]|19[0-3])cm)|((59|6[0-9]|7[0-6])in))$").unwrap()),
        (String::from("hcl"), Regex::new(r"^(#[0-9a-z]{6})$").unwrap()),
        (String::from("ecl"), Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap()),
        (String::from("pid"), Regex::new(r"^(\d{9})$").unwrap())
    ].into_iter().collect();

    let answer_two =
        input.iter()
            .filter(|p| p.len() == 7 && p.iter().all(|(f, v)| rules[f].is_match(v)))
            .count();

    println!("Part Two: {}", answer_two);
}
