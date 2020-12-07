use crate::puzzles::Puzzle;

struct Day01 { }

pub fn create() -> impl Puzzle {
    Day01 { }
}

impl Puzzle for Day01 {
    fn solve_part_one(&self, input: &Vec<String>) -> Option<String> {
        let input = parse_input(&input);

        find_sum(&input, 2020)
            .map(|(x, y)| (x * y).to_string())
    }

    fn solve_part_two(&self, input: &Vec<String>) -> Option<String> {
        let input = parse_input(input);

        input.iter().enumerate()
            .filter_map(|(i, x)| find_sum(&input[i + 1..], 2020 - x).map(|(y, z)| (x * y * z).to_string()))
            .next()
    }
}

fn parse_input(input: &[String]) -> Vec<i64> {
    let mut input = input.iter()
         .map(|x| x.parse::<i64>().unwrap())
         .collect::<Vec<_>>();
    
    input.sort();

    input
}

fn find_sum(expenses: &[i64], target: i64) -> Option<(i64, i64)> {
    let mut low : usize = 0;
    let mut high = expenses.len() - 1;

    while low < high {
        let x = expenses[low];
        
        while high > low {
            let y = expenses[high];
            let sum = x + y;

            if sum == target {
                return Some((x, y))
            }

            if sum > target {
                high = high - 1;
            }
            else {
                break;
            }
        }

        low = low + 1;
    }

    None
}