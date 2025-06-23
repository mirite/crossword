mod clue;
mod input_parser;

use clue::Clue;
use input_parser::read_clue;
use std::env;

use crate::clue::Direction;

fn main() {
    let mut args = env::args();
    args.next();
    let input_strings: Vec<String> = args.collect();
    let mut clues: Vec<Clue> = Vec::with_capacity(input_strings.len());
    for input in input_strings {
        clues.push(read_clue(input));
    }
    let grid_size = get_grid_size(&clues);
    for clue in clues {
        println!("{}", clue);
    }
    println!("{}x{}", grid_size.0, grid_size.1)
}

fn get_grid_size(clues: &Vec<Clue>) -> (u32, u32) {
    let mut max_x: u32 = 0;
    let mut max_y: u32 = 0;
    for clue in clues {
        match clue.direction {
            Direction::Down => {
                let y = clue.y + clue.answer.len() as u32;
                if y > max_y {
                    max_y = y;
                }
            }
            Direction::Across => {
                let x = clue.x + clue.answer.len() as u32;
                if x > max_x {
                    max_x = x;
                }
            }
        }
    }
    (max_x, max_y)
}
