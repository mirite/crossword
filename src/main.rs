mod clue;
mod grid;
mod input_parser;

use clue::Clue;
use input_parser::read_clue;
use std::io::{self, BufRead};

use crate::{clue::Direction, grid::Grid};

fn main() {
    let stdin = io::stdin();
    let mut clues: Vec<Clue> = Vec::new();

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                eprintln!("Error reading line from stdin: {}", e);
                continue;
            }
        };

        if line.trim().is_empty() {
            continue;
        }

        clues.push(read_clue(line));
    }

    if clues.is_empty() {
        eprintln!("No clues were provided. Please pipe in a text file with clues.");
        return;
    }
    let grid_size = get_grid_size(&clues);
    let grid = Grid {
        clues: clues,
        width: grid_size.0,
        height: grid_size.1,
    };
    println!("{}", grid);
}

fn get_grid_size(clues: &Vec<Clue>) -> (u8, u8) {
    let mut max_x: u8 = 0;
    let mut max_y: u8 = 0;
    for clue in clues {
        match clue.direction {
            Direction::Down => {
                let y = clue.y + clue.answer.len() as u8;
                if y > max_y {
                    max_y = y;
                }
            }
            Direction::Across => {
                let x = clue.x + clue.answer.len() as u8;
                if x > max_x {
                    max_x = x;
                }
            }
        }
    }
    (max_x, max_y)
}
