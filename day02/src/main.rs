use std::io;
use std::io::prelude::*;
use regex::Regex;

fn main() {
    let r = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
    let input : Vec<_> = 
        io::stdin().lock().lines()
               .map(|l| l.unwrap())
               .take_while(|l| l != "")
               .map(|l| {
                   r.captures(&l)
                    .map(|c| (
                        c[1].parse::<usize>().unwrap(), 
                        c[2].parse::<usize>().unwrap(),
                        c[3].chars().next().unwrap(),
                        c[4].to_owned()
                    )).unwrap()
               })
               .collect();


    let answer_one = 
        input.iter()
             .map(|(min, max, c, pwd)| (min, max, pwd.chars().filter(|x| x == c).count()))
             .filter(|(min, max, v)| min <= &v && &v <= max)
             .count();

    println!("Part One: {}", answer_one);

    let answer_two = 
        input.iter()
            .filter(|(min, max, c, pwd)| 
                pwd.chars().nth(min - 1).map(|x| &x == c).unwrap_or(false) ^
                pwd.chars().nth(max - 1).map(|x| &x == c).unwrap_or(false)
            )
            .count();

    println!("Part Two: {}", answer_two);
}
