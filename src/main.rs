mod clue;
mod input_parser;

use clue::Clue;
use input_parser::read_clue;
use std::env;

fn main() {
    let mut args = env::args();
    args.next();
    let hints: Vec<String> = args.collect();
    let mut clues: Vec<Clue> = Vec::new();
    for hint in hints {
        clues.push(read_clue(hint));
    }
    for clue in clues {
        println!("{}", clue);
    }
}
