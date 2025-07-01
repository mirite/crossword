use crate::{
    clue::Clue,
    placement::{Grid, SquareValue},
};

pub fn assign_clue_numbers(grid: &Grid, clues: &mut Vec<Clue>) {
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
