use crate::{clue::Direction, placement::Grid, placement::SquareValue};

/// Checks if a word can be placed on the grid at a given position and direction.
///
/// This function checks for:
/// 1. Grid boundaries: Does the word fit?
/// 2. Adjacency: Are the squares before and after the word empty or black?
/// 3. Intersections: Does it align with existing characters?
pub fn can_place(
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
            if x > 0 {
                if let SquareValue::Char(_) = grid[y][x - 1] {
                    return false; // Adjacent char to the left
                }
            }
            if x + word_len < grid_width {
                if let SquareValue::Char(_) = grid[y][x + word_len] {
                    return false; // Adjacent char to the right
                }
            }
        }
        Direction::Down => {
            if y > 0 {
                if let SquareValue::Char(_) = grid[y - 1][x] {
                    return false; // Adjacent char above
                }
            }
            if y + word_len < grid_height {
                if let SquareValue::Char(_) = grid[y + word_len][x] {
                    return false; // Adjacent char below
                }
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
