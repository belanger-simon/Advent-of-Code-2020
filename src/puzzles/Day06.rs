use crate::puzzles::Puzzle;
use std::collections::HashSet;

struct Day06 { }

pub fn create() -> impl Puzzle {
    Day06 { }
}

impl Puzzle for Day06 {
    fn solve_part_one(&self, input: &Vec<String>) -> Option<String> {
        let input = parse_input(input);
            
        Some(
            input.iter()
                 .map(|group| {
                     group.iter().fold(Questionnaire::new(), |l, r| l.union(r).cloned().collect()).len()
                 })
                 .sum::<usize>().to_string()
        )
    }

    fn solve_part_two(&self, input: &Vec<String>) -> Option<String> {
        let input = parse_input(input);
        
        Some(
            input.iter()
                 .map(|group| {
                    group.iter().fold(('a'..='z').collect::<Questionnaire>(), |l, r| l.intersection(r).cloned().collect()).len()
                 })
                 .sum::<usize>().to_string()
        )
    }
}

type Questionnaire = HashSet<char>;
type Group = Vec<Questionnaire>;

fn parse_input(input: &[String]) -> Vec<Group> {
    input.split(|l| l.len() == 0)
         .map(|group| {
             group
                .iter()
                .map(|q| q.chars().collect())
                .collect()
         })
         .collect()
}