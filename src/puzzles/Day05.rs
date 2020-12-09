use crate::puzzles::Puzzle;

struct Day05 { }

pub fn create() -> impl Puzzle {
    Day05 { }
}

impl Puzzle for Day05 {
    fn solve_part_one(&self, input: &Vec<String>) -> Option<String> {
        let input = parse_input(&input);

        Some(input.iter().map(|p| p.row * 8 + p.col).max().unwrap().to_string())
    }

    fn solve_part_two(&self, input: &Vec<String>) -> Option<String> {
        let input = parse_input(input);
        
        let mut ids : Vec<u32> = input.iter().map(|p| p.row * 8 + p.col).collect();
        ids.sort();

        ids.iter()
           .skip(1)
           .zip(ids.iter())
           .filter(|(x, y)| *x - *y == 2)
           .map(|(x,_)| (x - 1).to_string())
           .next()
    }
}

struct BoardingPass {
    row: u32,
    col: u32
}

impl BoardingPass {
    fn from(id: &str) -> BoardingPass {
        BoardingPass {
            row: id.chars()
                   .take(7)
                   .fold((0, 127), |(min, max), c| if c == 'F' { (min, max - (max - min) / 2 - 1) } else { (min + (max - min) / 2 + 1, max) })
                   .0,
            col: id.chars()
                   .skip(7)
                   .fold((0, 7), |(min, max), c| if c == 'L' { (min, max - (max - min) / 2 - 1) } else { (min + (max - min) / 2 + 1, max) })
                   .0,
        }
    }
}

fn parse_input(input: &[String]) -> Vec<BoardingPass> {
    input.iter().map(|id| BoardingPass::from(id)).collect()
}