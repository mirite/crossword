use std::fmt::{Display, Formatter};

use crate::clue::{Clue, Direction};

const SQUARE_SIZE: usize = 32;
const FONT_SIZE: f32 = 12.0;

pub struct Grid {
    pub clues: Vec<Clue>,
    pub width: u8,
    pub height: u8,
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let viewbox_width: usize = self.width as usize * SQUARE_SIZE;
        let viewbox_height: usize = self.height as usize * SQUARE_SIZE;

        let rendered_squares = render_squares(&self.clues);
        let rendered_answers = render_answers(&self.clues);

        write!(
            f,
            "<svg viewbox=\"0 0 {} {}\" xmlns=\"http://www.w3.org/2000/svg\">{}</svg>",
            viewbox_width,
            viewbox_height,
            format!("{}{}", rendered_squares, rendered_answers)
        )
    }
}

/// Renders the SVG group for the letters (answers) on the grid.
fn render_answers(clues: &[Clue]) -> String {
    let content: String = clues.iter().map(render_answer).collect();

    format!("<g id=\"answers\">{}</g>", content)
}

/// Renders the SVG group for the squares of the grid.
fn render_squares(clues: &[Clue]) -> String {
    let content: String = clues.iter().map(render_clue_squares).collect::<String>();

    format!("<g id=\"grid\">{}</g>", content)
}
/// Renders the SVG squares and clue number for a single clue.
fn render_clue_squares(clue: &Clue) -> String {
    let mut content = String::new();
    let mut x = clue.x;
    let mut y = clue.y;

    for (index, _) in clue.answer.chars().enumerate() {
        content.push_str(&render_square(x, y));

        // Only add the clue number for the first square.
        if index == 0 {
            content.push_str(&render_text(
                &clue.number.to_string(),
                x as usize * SQUARE_SIZE + 3,
                y as usize * SQUARE_SIZE + (SQUARE_SIZE / 2) - 10,
                Some(FONT_SIZE / 2.0),
            ));
        }

        match clue.direction {
            Direction::Down => y += 1,
            Direction::Across => x += 1,
        }
    }
    content
}
/// Renders the SVG text for the letters of a single clue's answer.
fn render_answer(clue: &Clue) -> String {
    let mut content = String::new();
    let mut x = clue.x;
    let mut y = clue.y;

    for letter in clue.answer.chars() {
        content.push_str(&render_square_answer(letter, x, y));
        match clue.direction {
            Direction::Down => y += 1,
            Direction::Across => x += 1,
        }
    }
    content
}
/// Renders a single white square with a black stroke.
fn render_square(x: u8, y: u8) -> String {
    let x_root = x as usize * SQUARE_SIZE;
    let y_root = y as usize * SQUARE_SIZE;
    format!(
        "<rect width=\"{}\" height=\"{}\" x=\"{}\" y=\"{}\" fill=\"#FFFFFF\" stroke=\"#000000\"></rect>",
        SQUARE_SIZE, SQUARE_SIZE, x_root, y_root
    )
}
fn render_square_answer(letter: char, x: u8, y: u8) -> String {
    let x_centre = x as usize * SQUARE_SIZE + (SQUARE_SIZE / 2);
    let y_centre = y as usize * SQUARE_SIZE + (SQUARE_SIZE / 2) + 5;
    render_text(
        letter.to_string().as_str(),
        x_centre,
        y_centre,
        Some(FONT_SIZE),
    )
}
/// Renders an SVG text element.
fn render_text(text: &str, x: usize, y: usize, font_size: Option<f32>) -> String {
    let font_size_attr = if let Some(size) = font_size {
        format!("font-size=\"{}\"", size)
    } else {
        String::new()
    };

    format!(
        "<text stroke=\"#000000\" x=\"{}\" y=\"{}\" text-anchor=\"middle\" dominant-baseline=\"middle\" {}>{}</text>",
        x, y, font_size_attr, text
    )
}
