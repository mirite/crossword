use crate::{
    clue::{Clue, Direction},
    clue_sorting::sort,
};

#[derive(Clone, Copy, Debug, PartialEq)]
enum SquareValue {
    Char(char),
    Blank,
    Black,
}
type Grid = Vec<Vec<SquareValue>>;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct BaseClue {
    pub clue: String,
    pub answer: String,
}

pub fn place_clues(lines: Vec<String>) -> Vec<Clue> {
    let pairs = lines
        .iter()
        .map(|line| {
            let parts: Vec<&str> = line.split(";").collect();
            BaseClue {
                clue: parts[0].to_string(),
                answer: parts[1].to_string(),
            }
        })
        .collect();
    let sorted = sort(pairs);
    let size: usize = sorted[0].answer.len();
    let initial_grid = vec![vec![SquareValue::Blank; size]; size];
    let mut clues: Vec<Clue> = vec![];
    place_word(0, initial_grid, &sorted, &mut clues);
    clues
}

fn place_word(
    word_index: usize,
    grid: Grid,
    clues: &Vec<BaseClue>,
    result: &mut Vec<Clue>,
) -> bool {
    if word_index == clues.len() {
        return true;
    }
    let is_first_word = word_index == 0;
    let current_clue = &clues[word_index];
    let current_answer = &current_clue.answer;
    let height = grid.len();
    let width = grid[0].len();
    if is_first_word {
        let (x, y, direction) = (0, 0, Direction::Across);
        let next_grid = write_word_to_grid(&grid, current_answer, x, y, &direction);
        result.push(Clue {
            base: BaseClue {
                clue: current_clue.clue.clone(),
                answer: current_answer.clone(),
            },
            x: x as u8,
            y: y as u8,
            direction,
            number: 1,
        });

        if place_word(word_index + 1, next_grid, clues, result) {
            return true;
        }

        result.pop(); // Backtrack
    }
    for y in 0..height {
        for x in 0..width {
            for direction in [Direction::Across, Direction::Down] {
                if can_place(current_answer, &grid, x, y, &direction, false) {
                    let next_grid = write_word_to_grid(&grid, current_answer, x, y, &direction);
                    result.push(Clue {
                        base: BaseClue {
                            clue: current_clue.clue.clone(),
                            answer: current_answer.clone(),
                        },
                        x: x as u8,
                        y: y as u8,
                        direction,
                        number: 1,
                    });

                    if place_word(word_index + 1, next_grid, clues, result) {
                        return true;
                    }

                    result.pop();
                }
            }
        }
    }
    false
}

fn write_word_to_grid(grid: &Grid, word: &str, x: usize, y: usize, direction: &Direction) -> Grid {
    let mut new_grid = grid.clone();
    for (i, char) in word.chars().enumerate() {
        match direction {
            Direction::Across => new_grid[y][x + i] = SquareValue::Char(char),
            Direction::Down => new_grid[y + i][x] = SquareValue::Char(char),
        }
    }
    new_grid
}
/// Checks if a word can be placed on the grid at a given position and direction.
///
/// This function checks for:
/// 1. Grid boundaries: Does the word fit?
/// 2. Adjacency: Are the squares before and after the word empty or black?
/// 3. Intersections: Does it align with existing characters?
fn can_place(
    answer: &str,
    grid: &Grid,
    x: usize,
    y: usize,
    direction: &Direction,
    is_grid_empty: bool,
) -> bool {
    let word_len = answer.len();
    let grid_height = grid.len();
    let grid_width = grid[0].len();
    match direction {
        Direction::Across if x + word_len > grid_width => return false,
        Direction::Down if y + word_len > grid_height => return false,
        _ => (),
    }

    match direction {
        Direction::Across => {
            if x > 0 && grid[y][x - 1] != SquareValue::Black {
                return false;
            }
            if x + word_len < grid_width && grid[y][x + word_len] != SquareValue::Black {
                return false;
            }
        }
        Direction::Down => {
            if y > 0 && grid[y - 1][x] != SquareValue::Black {
                return false;
            }
            if y + word_len < grid_height && grid[y + word_len][x] != SquareValue::Black {
                return false;
            }
        }
    }

    let mut intersects = false;
    for (i, answer_char) in answer.chars().enumerate() {
        let (curr_x, curr_y) = match direction {
            Direction::Across => (x + i, y),
            Direction::Down => (x, y + i),
        };

        // 5. --- Parallelism Check ---
        // For each letter, check the perpendicular cells. If they contain a character,
        // it means we are placing a word right next to an existing one, which is invalid.
        // The only time a perpendicular cell can have a character is if it's an intersection.
        let is_intersection = matches!(grid[curr_y][curr_x], SquareValue::Char(_));

        if !is_intersection {
            match direction {
                Direction::Across => {
                    if curr_y > 0 && matches!(grid[curr_y - 1][curr_x], SquareValue::Char(_)) {
                        return false;
                    }
                    if curr_y < grid_height - 1
                        && matches!(grid[curr_y + 1][curr_x], SquareValue::Char(_))
                    {
                        return false;
                    }
                }
                Direction::Down => {
                    if curr_x > 0 && matches!(grid[curr_y][curr_x - 1], SquareValue::Char(_)) {
                        return false;
                    }
                    if curr_x < grid_width - 1
                        && matches!(grid[curr_y][curr_x + 1], SquareValue::Char(_))
                    {
                        return false;
                    }
                }
            }
        }

        // 3. --- Intersection Check ---
        match grid[curr_y][curr_x] {
            SquareValue::Black => return false, // Can't place over a black square.
            SquareValue::Blank => { /* This spot is open, which is fine. */ }
            SquareValue::Char(existing_char) => {
                if existing_char == answer_char {
                    intersects = true; // This is a valid intersection.
                } else {
                    return false; // The characters do not match.
                }
            }
        }
    }

    // 4. --- Connectivity Check ---
    // The placement is valid only if it's the first word on the grid OR
    // it connects to at least one existing word.
    is_grid_empty || intersects
}
