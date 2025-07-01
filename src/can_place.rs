use crate::{clue::Direction, placement::Grid, placement::SquareValue};

/// Checks if a word can be placed on the grid at a given position and direction.
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
                    return false;
                }
            }
            if x + word_len < grid_width {
                if let SquareValue::Char(_) = grid[y][x + word_len] {
                    return false;
                }
            }
        }
        Direction::Down => {
            if y > 0 {
                if let SquareValue::Char(_) = grid[y - 1][x] {
                    return false;
                }
            }
            if y + word_len < grid_height {
                if let SquareValue::Char(_) = grid[y + word_len][x] {
                    return false;
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

        match grid[curr_y][curr_x] {
            SquareValue::Black => return false,
            SquareValue::Blank => { /* This spot is open, which is fine. */ }
            SquareValue::Char(existing_char) => {
                if existing_char == answer_char {
                    intersects = true;
                } else {
                    return false;
                }
            }
        }
    }

    // The placement is valid only if it's the first word on the grid OR
    // it connects to at least one existing word.
    is_grid_empty || intersects
}
