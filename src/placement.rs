use crate::{
    can_place::can_place,
    clue::{Clue, Direction},
    clue_numbering::assign_clue_numbers,
    clue_sorting::sort,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SquareValue {
    Char(char),
    Blank,
    Black,
}
pub type Grid = Vec<Vec<SquareValue>>;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct BaseClue {
    pub clue: String,
    pub answer: String,
}

pub fn place_clues(lines: Vec<String>) -> Result<Vec<Clue>, String> {
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
    for bonus_squares in 0..sorted[0].answer.len() * 2 {
        let size: usize = sorted[0].answer.len() + bonus_squares;
        let initial_grid = vec![vec![SquareValue::Blank; size]; size];
        let mut clues: Vec<Clue> = vec![];
        let result_for_size = place_word(0, initial_grid, &sorted, &mut clues);
        if result_for_size.is_ok() == true {
            assign_clue_numbers(&result_for_size.unwrap(), &mut clues);
            return Ok(clues);
        }
    }
    Err(format!(
        "Failed to place the {} clues. The longest word was {}.",
        sorted.len(),
        sorted[0].answer
    ))
}

fn place_word(
    word_index: usize,
    grid: Grid,
    clues: &Vec<BaseClue>,
    result: &mut Vec<Clue>,
) -> Result<Grid, bool> {
    if word_index == clues.len() {
        return Ok(grid);
    }
    let current_clue = &clues[word_index];
    let current_answer = &current_clue.answer;
    let height = grid.len();
    let width = grid[0].len();
    for y in 0..height {
        for x in 0..width {
            for direction in [Direction::Across, Direction::Down] {
                if can_place(current_answer, &grid, x, y, &direction, word_index == 0) {
                    let next_grid = write_word_to_grid(&grid, current_answer, x, y, &direction);
                    result.push(Clue {
                        base: current_clue.clone(),
                        x,
                        y,
                        direction,
                        number: 0,
                    });
                    let next_result = place_word(word_index + 1, next_grid, clues, result);
                    if next_result.is_ok() == true {
                        return next_result;
                    }

                    result.pop();
                }
            }
        }
    }
    Err(false)
}

/// Block off the squares the word should occupy.
fn write_word_to_grid(grid: &Grid, word: &str, x: usize, y: usize, direction: &Direction) -> Grid {
    let mut new_grid = grid.clone();
    for (i, char) in word.chars().enumerate() {
        match direction {
            Direction::Across => new_grid[y][x + i] = SquareValue::Char(char),
            Direction::Down => new_grid[y + i][x] = SquareValue::Char(char),
        }
    }
    match direction {
        Direction::Across => {
            if x > 0 {
                new_grid[y][x - 1] = SquareValue::Black;
            }
            if x + word.len() < grid[0].len() - 1 {
                new_grid[y][x + word.len()] = SquareValue::Black;
            }
        }
        Direction::Down => {
            if y > 0 {
                new_grid[y - 1][x] = SquareValue::Black;
            }
            if y + word.len() < grid.len() - 1 {
                new_grid[y + word.len()][x] = SquareValue::Black;
            }
        }
    }
    new_grid
}
