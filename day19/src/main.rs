use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug)]
enum Rule {
    Char(char),
    Rule(Vec<usize>)
}

type Rules = HashMap<usize, Vec<Rule>>;

fn match_message(rules: &Rules, label: usize, message: &Vec<char>) -> bool {
    fn match_message(rules: &Rules, mut to_match: VecDeque<usize>, message: &[char]) -> bool {
        match (to_match.len(), message.len()) {
            (0, 0) => true,
            (_, 0) | (0, _) => false,
            _ => {
                for rule in &rules[&to_match.pop_front().unwrap()] {
                    let matched = match rule {
                        Rule::Char(c) => &message[0] == c && match_message(rules, to_match.clone(), &message[1..]),
                        Rule::Rule(labels) => {
                            if labels.len() <= message.len() {
                                match_message(rules, labels.iter().chain(to_match.iter()).copied().collect(), message)
                            }
                            else {
                                false
                            }
                        }
                    };

                    if matched {
                        return true
                    }
                }
                
                false
            }
        }
    }
    
    match_message(rules, [label].iter().copied().collect(), &message[..])
}

fn main() {
    let mut rules : Rules = io::stdin().lock().lines()
        .map(|l| l.unwrap())
        .take_while(|l| l != "")
        .map(|l| {
            let mut parts = l.split(": ");
            let label = parts.next().unwrap().parse::<usize>().unwrap();
            let rule = parts.next().unwrap();
            
            if rule.starts_with('"') {
                (label, vec![Rule::Char(rule.chars().nth(1).unwrap())])
            } 
            else {
                (label, rule.split(" | ").map(|r| Rule::Rule(r.split(" ").map(|d| d.parse::<usize>().unwrap()).collect())).collect())
            }
        })
        .collect();

    let messages : Vec<Vec<_>> = io::stdin().lock().lines()
        .map(|l| l.unwrap())
        .take_while(|l| l != "")
        .map(|l| l.chars().collect())
        .collect();
    
    let answer_one = messages.iter().filter(|m| match_message(&rules, 0, m)).count();

    println!("Part One: {}", answer_one);

    rules.insert(8, vec![Rule::Rule(vec![42]), Rule::Rule(vec![42,8])]);
    rules.insert(11, vec![Rule::Rule(vec![42,31]), Rule::Rule(vec![42,11,31])]);
    
    let answer_two = messages.iter().filter(|m| match_message(&rules, 0, m)).count();

    println!("Part Two: {}", answer_two);
}