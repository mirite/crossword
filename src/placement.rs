use crate::{
    can_place::can_place,
    clue::{Clue, Direction},
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
    for bonus_squares in 0..sorted[0].answer.len() * 2 {
        let size: usize = sorted[0].answer.len() + bonus_squares;
        let initial_grid = vec![vec![SquareValue::Blank; size]; size];
        let mut clues: Vec<Clue> = vec![];
        let mut final_grid = initial_grid.clone();
        let success = place_word(0, initial_grid, &sorted, &mut clues, &mut final_grid);
        if success == true {
            assign_clue_numbers(&final_grid, &mut clues);
            return clues;
        }
    }
    panic!(
        "Failed to place the {} clues. The longest word was {}.",
        sorted.len(),
        sorted[0].answer
    );
}

fn place_word(
    word_index: usize,
    grid: Grid, // This grid is for the current recursive step
    clues: &Vec<BaseClue>,
    result: &mut Vec<Clue>,
    final_grid_ref: &mut Grid, // New parameter to pass the final grid back
) -> bool {
    if word_index == clues.len() {
        *final_grid_ref = grid; // Assign the current successful grid to the final_grid_ref
        return true;
    }
    let is_first_word = word_index == 0;
    let current_clue = &clues[word_index];
    let current_answer = &current_clue.answer;
    let height = grid.len();
    let width = grid[0].len();
    for y in 0..height {
        for x in 0..width {
            for direction in [Direction::Across, Direction::Down] {
                if can_place(current_answer, &grid, x, y, &direction, is_first_word) {
                    let next_grid = write_word_to_grid(&grid, current_answer, x, y, &direction);
                    result.push(Clue {
                        base: BaseClue {
                            clue: current_clue.clue.clone(),
                            answer: current_answer.clone(),
                        },
                        x: x as u8,
                        y: y as u8,
                        direction,
                        number: 0, // Initialize with 0 or a placeholder, it will be updated later
                    });

                    // Pass the final_grid_ref down the recursion
                    if place_word(word_index + 1, next_grid, clues, result, final_grid_ref) {
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

fn assign_clue_numbers(grid: &Grid, clues: &mut Vec<Clue>) {
    let height = grid.len();
    let width = grid[0].len();
    let mut current_number = 1;
    let mut numbered_cells: Vec<Vec<u8>> = vec![vec![0; width]; height]; // Stores the assigned number for each cell

    // First pass: Determine which cells get a number
    // and assign the numbers to the numbered_cells grid
    for y in 0..height {
        for x in 0..width {
            let mut is_start_of_across = false;
            let mut is_start_of_down = false;

            if let SquareValue::Char(_) = grid[y][x] {
                // Only consider squares that contain a character
                // Check if it's the start of an Across word
                // It's the start if the square to its left is Black or out of bounds (x == 0)
                // AND there's a character in the current square.
                // We also need to ensure there IS an Across word, meaning at least two squares.
                if (x == 0 || grid[y][x - 1] == SquareValue::Black)
                    && (x + 1 < width && matches!(grid[y][x + 1], SquareValue::Char(_)))
                {
                    is_start_of_across = true;
                }

                // Check if it's the start of a Down word
                // It's the start if the square above it is Black or out of bounds (y == 0)
                // AND there's a character in the current square.
                // We also need to ensure there IS a Down word, meaning at least two squares.
                if (y == 0 || grid[y - 1][x] == SquareValue::Black)
                    && (y + 1 < height && matches!(grid[y + 1][x], SquareValue::Char(_)))
                {
                    is_start_of_down = true;
                }
            }

            // A square gets a number if it's the start of *either* an Across or Down word
            if is_start_of_across || is_start_of_down {
                numbered_cells[y][x] = current_number as u8;
                current_number += 1;
            }
        }
    }

    // Second pass: Assign these numbers to the actual Clue objects
    // Now, we need to iterate through the clues that were *actually placed*
    // and assign them their corresponding number from numbered_cells.
    for clue in clues.iter_mut() {
        // Only assign a number if the cell in numbered_cells is not 0 (meaning it's a valid starting point)
        clue.number = numbered_cells[clue.y as usize][clue.x as usize];
    }

    // Sort the clues by their assigned number for proper display
    clues.sort_by_key(|c| c.number);
}
