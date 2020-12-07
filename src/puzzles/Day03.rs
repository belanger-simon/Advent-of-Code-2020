use crate::puzzles::Puzzle;

struct Day03 { }

pub fn create() -> impl Puzzle {
    Day03 { }
}

impl Puzzle for Day03 {
    fn solve_part_one(&self, input: &Vec<String>) -> Option<String> {
        let map = parse_input(&input);

        Some(traverse_map(&map, &Position(3, 1)).to_string())
    }

    fn solve_part_two(&self, input: &Vec<String>) -> Option<String> {
        let map = parse_input(input);

        let slopes = vec![Position(1, 1), Position(3, 1), Position(5, 1), Position(7, 1), Position(1, 2)];

        Some(Iterator::product::<u32>(slopes.iter()
                   .map(|slope| traverse_map(&map, slope)))
                   .to_string())
    }
}

fn parse_input(input: &[String]) -> Map {
    let grid = input.iter()
        .map(|row| row.chars().map(|c| if c == '.' { Terrain::Open } else { Terrain::Tree }).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    Map { grid }
}

struct Position(usize, usize);

impl Position {
    fn translate(&self, position: &Position) -> Position {
        Position(self.0 + position.0, self.1 + position.1)
    }
}

enum Terrain {
    Open,
    Tree
}

struct Map {
    grid: Vec<Vec<Terrain>>
}

impl Map {
    fn get(&self, position: &Position) -> Option<&Terrain> {
        match self.grid.get(position.1) {
            Some(row) => row.get(position.0 % row.len()),
            None => None
        }
    }
}

fn traverse_map(map: &Map, slope: &Position) -> u32 {
    let mut position = Position(0, 0);
    let mut count = 0;

    while let Some(cell) = map.get(&position) {
        match cell {
            Terrain::Tree => count = count + 1,
            Terrain::Open => (),
        }
        
        position = position.translate(slope);
    }

    count
}