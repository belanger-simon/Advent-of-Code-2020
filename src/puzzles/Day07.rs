use crate::puzzles::Puzzle;
use std::collections::{VecDeque, HashMap};
use regex::Regex;

struct Day07 { }

pub fn create() -> impl Puzzle {
    Day07 { }
}

impl Puzzle for Day07 {
    fn solve_part_one(&self, input: &Vec<String>) -> Option<String> {
        let graph = parse_input(input);
        let target = String::from("shiny gold");
        let mut visited : HashMap<&String, bool> = HashMap::new();
        let mut to_visit : VecDeque<&String> = graph.vertices.keys().filter(|v| **v != target).collect();

        visited.insert(&target, true);

        while !to_visit.is_empty() {
            let label = to_visit.pop_front().unwrap();
            let edges = &graph.vertices.get(label).unwrap().edges;

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

        Some(visited.iter().filter(|(k, v)| ***k != target && **v).count().to_string())
    }

    fn solve_part_two(&self, input: &Vec<String>) -> Option<String> {
        let graph = parse_input(input);
        let target = String::from("shiny gold");
        let mut visited : HashMap<&String, u32> = HashMap::new();
        let mut to_visit : VecDeque<&String> = VecDeque::new();

        to_visit.push_back(&target);

        while let Some(current) = to_visit.pop_front() {
            if visited.contains_key(current) {
                continue;
            }

            let v = graph.vertices.get(current).unwrap();
            let mut any_missing = false;
            let mut count = 0;

            for (l, c) in &v.edges {
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

        visited.get(&target).map(|v| v.to_string())
    }
}

fn parse_input(input: &[String]) -> Graph {
    input.iter()
        .map(parse_rule)
        .collect()
}

fn parse_rule(rule: &String) -> Rule {
    let r = Regex::new(r"^(\d+) (.*) bags?\.?").unwrap();
    let mut iter = rule.split(" bags contain ");

    (
        iter.next().unwrap().to_owned(),
        iter.next().unwrap().split(", ")
            .filter_map(|x| r.captures(x).map(|c| (c[2].to_owned(), c[1].parse::<u32>().unwrap())))
            .collect()
    )
}

type Rule = (String, Vec<(String, u32)>);

struct Vertice {
    edges: HashMap<String, u32>
}

impl Vertice {
    fn new() -> Vertice {
        Vertice { 
            edges: HashMap::new()
        }
    }

    fn add_edge(&mut self, to_label: &String, count: &u32) {
        self.edges.insert(to_label.to_owned(), *count);
    }
}

struct Graph {
    vertices: HashMap<String, Vertice>,
}

impl Graph {
    fn new() -> Self {
        Self {
            vertices: HashMap::new()
        }
    }

    fn add_vertice(&mut self, label: &String) -> &mut Vertice {
        self.vertices.insert(label.to_owned(), Vertice::new());

        self.vertices.get_mut(label).unwrap()
    }
}

impl std::iter::FromIterator<Rule> for Graph {
    fn from_iter<I: IntoIterator<Item=Rule>>(iter: I) -> Self {
        let mut graph = Graph::new();

        for v in iter {
            let vertice = &mut graph.add_vertice(&v.0);

            for e in v.1.iter() {
                vertice.add_edge(&e.0, &e.1);
            }
        }

        graph
    }
}