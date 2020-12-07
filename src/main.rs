use std::io;
use std::io::prelude::*;
mod puzzles;

fn main() {
    let day = read_day();
    let puzzle = puzzles::get_puzzle(day)
                         .expect("No implementation for the puzzle found");

    let input = read_input();
    
    println!("Part One");

    match puzzle.solve_part_one(&input) {
        Some(result) => println!("{}", result),
        None => println!("Not implemented")
    }

    println!("\nPart two");

    match puzzle.solve_part_two(&input) {
        Some(result) => println!("{}", result),
        None => println!("Not implemented")
    }
}

fn read_day() -> u8 {
    let mut input = String::new();

    print!("Enter the day of the solution: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    input.trim_end()
         .parse()
         .expect("Can't extract day from input")
}

fn read_input() -> Vec<String> {

    println!("Write the input to the puzzle. Enter an empty line to end the input");

    io::stdin().lock()
        .lines()
        .map(|l| l.unwrap().to_owned())
        .take_while(|l| l.len() > 0)
        .collect()
}