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
    let lines: Vec<String> = stdin.lock().lines().filter_map(Result::ok).collect();
    match place_clues(lines) {
        Ok(clues) => {
            let grid_size = get_grid_size(&clues);
            let grid = Grid {
                clues: clues,
                width: grid_size.0,
                height: grid_size.1,
            };
            println!("{}", grid);
        }
        Err(e) => println!("Failed to generate grid: {}", e),
    }
}

fn get_grid_size(clues: &Vec<Clue>) -> (usize, usize) {
    let mut max_x: usize = 1;
    let mut max_y: usize = 1;
    for clue in clues {
        match clue.direction {
            Direction::Down => {
                let current_y_extent = clue.y + clue.base.answer.len();
                max_y = max_y.max(current_y_extent);
                max_x = max_x.max(clue.x + 1); // +1 because x is a coordinate, and we need the dimension
            }
            Direction::Across => {
                let current_x_extent = clue.x + clue.base.answer.len();
                max_y = max_y.max(clue.y + 1);
                max_x = current_x_extent;
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
                x: 9,
                y: 9,
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
