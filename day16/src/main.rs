use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use regex::Regex;

fn main() {
    let r = Regex::new(r"^([^:]+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();

    let rules : HashMap<_,_> = 
        io::stdin().lock()
            .lines()
            .map(|l| l.unwrap())
            .take_while(|l| l != "")
            .map(|l| r.captures(&l).map(|c| (c[1].to_owned(), (c[2].parse::<u32>().unwrap()..=c[3].parse::<u32>().unwrap(), c[4].parse::<u32>().unwrap()..=c[5].parse::<u32>().unwrap()))).unwrap())
            .collect();

    let tickets : Vec<_> = 
        io::stdin().lock()
            .lines()
            .map(|l| l.unwrap())
            .skip(1).next()
            .map(|l| l.split(',').map(|n| n.parse::<u32>().unwrap()).collect()).unwrap();

    let tickets : Vec<Vec<_>> =
        vec![tickets].into_iter().chain(
        io::stdin().lock()
            .lines()
            .skip(2)
            .map(|l| l.unwrap())
            .take_while(|l| l != "")
            .map(|l| l.split(',').map(|n| n.parse::<u32>().unwrap()).collect()))
            .collect();

    let possibilities : Vec<Vec<HashSet<_>>> = tickets.iter()
            .map(|t| t.iter()
                      .map(|f| rules.iter()
                                    .filter_map(|(n, (r1, r2))| if r1.contains(f) || r2.contains(f) { Some(n) } else { None })
                                    .collect())
                      .collect())
            .collect();
    
    let answer_one = possibilities.iter()
        .enumerate()
        .map(|(i, t)| {
            t.iter().enumerate()
                .filter_map(|(j, p)| if p.len() == 0 { Some(tickets[i][j]) } else { None })
                .sum::<u32>()
        })
        .sum::<u32>();

    println!("Part One: {}", answer_one);

    let possibilities : Vec<Vec<HashSet<_>>> = possibilities.into_iter().filter(|t| t.iter().all(|p| p.len() > 0)).collect();
    let mut to_find : HashSet<&String> = rules.keys().collect();
    let mut ticket_fields : Vec<Option<&String>> = vec![None; rules.len()];

    while !to_find.is_empty() { 
        for (i, field) in ticket_fields.iter_mut().enumerate().filter(|(_, f)| f.is_none()) {
            let possible_fields = possibilities.iter().fold(&to_find | &to_find, |r, p| &r & &p[i]);
            if possible_fields.len() == 1 {
                let rule = *possible_fields.iter().next().unwrap();
                *field = Some(rule);
                to_find.remove(rule);
            }
        }
    }

    let answer_two = ticket_fields.iter()
        .enumerate()
        .filter_map(|(i, r)| r.map(|r| if r.len() > 9 && &r[0..9] == "departure" { Some(tickets[0][i] as u64) }else { None }).unwrap())
        .product::<u64>();

    println!("Part Two: {}", answer_two);
}