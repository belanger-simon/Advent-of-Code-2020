use std::io;
use std::io::prelude::*;
use std::collections::{HashMap, VecDeque};
use regex::Regex;

fn main() {
    let r = Regex::new(r"^(\d+) (.*) bags?\.?").unwrap();
    let input : HashMap<String, HashMap<String, u32>> = 
        io::stdin().lock().lines()
                   .map(|l| l.unwrap())
                   .take_while(|l| l != "")
                   .map(|l| l.to_owned())
                   .map(|l| {
                       let mut l = l.split(" bags contain ");
                       (
                           l.next().unwrap().to_owned(),
                           l.next().unwrap().split(", ")
                            .filter_map(|x| r.captures(x).map(|c| (c[2].to_owned(), c[1].parse::<u32>().unwrap())))
                            .collect()
                       )
                   })
                   .collect();

    let target = String::from("shiny gold");
    let answer_one = part_one(&input, &target);

    println!("Part One: {}", answer_one);

    let answer_two = part_two(&input, &target);

    println!("Part Two: {}", answer_two);
}

fn part_one(input: &HashMap<String, HashMap<String, u32>>, target: &String) -> usize {
    let mut visited : HashMap<&String, bool> = HashMap::new();
    let mut to_visit : VecDeque<&String> = input.keys().filter(|v| *v != target).collect();

    visited.insert(target, true);

    while !to_visit.is_empty() {
        let label = to_visit.pop_front().unwrap();
        let edges = input.get(label).unwrap();

        if edges.len() == 0 {
            visited.insert(label, false);
        }
        else if edges.keys().all(|e| visited.contains_key(e)) {
            visited.insert(label, edges.keys().any(|e| *visited.get(e).unwrap()));
        }
        else {
            to_visit.push_back(label);
        }
    }

    visited.iter().filter(|(k, v)| **k != target && **v).count()
}

fn part_two(input: &HashMap<String, HashMap<String, u32>>, target: &String) -> u32 {
    let mut visited : HashMap<&String, u32> = HashMap::new();
    let mut to_visit : VecDeque<&String> = VecDeque::new();

    to_visit.push_back(&target);

    while let Some(current) = to_visit.pop_front() {
        if visited.contains_key(current) {
            continue;
        }

        let v = input.get(current).unwrap();
        let mut any_missing = false;
        let mut count = 0;

        for (l, c) in v.iter() {
            match visited.get(&l) {
                Some(total) => count += c + c * total,
                None => {
                    to_visit.push_back(&l);
                    any_missing = true;
                }
            }
        }

        if any_missing {
            to_visit.push_back(&current)
        }
        else {
            visited.insert(current, count);
        }
    }

    *visited.get(target).unwrap()
}