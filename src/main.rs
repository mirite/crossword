mod can_place;
mod clue;
mod clue_numbering;
mod clue_sorting;
mod placement;
mod render;

use clue::Clue;
use std::io::{self, BufRead};

use crate::{clue::Direction, placement::place_clues, render::Grid};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().filter_map(|l| l.ok()).collect();
    let clues: Vec<Clue> = place_clues(lines);
    let grid_size = get_grid_size(&clues);
    let grid = Grid {
        clues: clues,
        width: grid_size.0,
        height: grid_size.1,
    };
    println!("{}", grid);
}

fn get_grid_size(clues: &Vec<Clue>) -> (usize, usize) {
    let mut max_x: usize = 1;
    let mut max_y: usize = 1;
    for clue in clues {
        match clue.direction {
            Direction::Down => {
                let y = clue.y + clue.base.answer.len() - 1;
                if y > max_y {
                    max_y = y;
                }
                if clue.x > max_x {
                    max_x = clue.x;
                }
            }
            Direction::Across => {
                let x = clue.x + clue.base.answer.len() - 1;
                if x > max_x {
                    max_x = x;
                }
                if clue.y > max_y {
                    max_y = clue.y;
                }
            }
        }
    }
    (max_x, max_y)
}

#[cfg(test)]
mod tests {
    use crate::placement::BaseClue;

    use super::*;

    #[test]
    fn defaults_correctly() {
        let (x, y) = get_grid_size(&vec![]);
        assert_eq!(x, 1);
        assert_eq!(y, 1);
    }

    #[test]
    fn counts_correctly() {
        let (x, y) = get_grid_size(&vec![
            Clue {
                direction: Direction::Down,
                x: 10,
                y: 10,
                base: BaseClue {
                    clue: String::from("Clue"),
                    answer: String::from("ANSWER"),
                },
                number: 2,
            },
            Clue {
                direction: Direction::Down,
                x: 0,
                y: 0,
                base: BaseClue {
                    clue: String::from("Clue"),
                    answer: String::from("ANSWERTWO"),
                },
                number: 1,
            },
        ]);
        assert_eq!(x, 10);
        assert_eq!(y, 15);
    }
}
