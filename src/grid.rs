use std::fmt::{Display, Formatter};

use crate::clue::{Clue, Direction};

const SQUARE_SIZE: usize = 32;

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
            "<?xml version=\"1.0\" encoding=\"utf-8\"?><svg viewBox=\"0 0 {} {}\" xmlns=\"http://www.w3.org/2000/svg\"><style>text{{dominant-baseline:middle;}}.square{{fill:#FFFFFF;stroke:#000000;}}.answer{{ text-anchor:middle; font-family:Arial, Helvetica, sans-serif; font-weight:lighter;font-size:14px;}}.number{{font-size:8px;}}</style>{}</svg>",
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
                x as usize * SQUARE_SIZE + 5,
                y as usize * SQUARE_SIZE + (SQUARE_SIZE / 2) - 10,
                Some(String::from("number")),
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
        "<rect width=\"{}\" height=\"{}\" x=\"{}\" y=\"{}\" class=\"square\"></rect>",
        SQUARE_SIZE, SQUARE_SIZE, x_root, y_root
    )
}
fn render_square_answer(letter: char, x: u8, y: u8) -> String {
    let x_centre = x as usize * SQUARE_SIZE + (SQUARE_SIZE / 2);
    let y_centre = y as usize * SQUARE_SIZE + (SQUARE_SIZE / 2) + 1;
    render_text(
        letter.to_string().as_str(),
        x_centre,
        y_centre,
        Some(String::from("answer")),
    )
}
/// Renders an SVG text element.
fn render_text(text: &str, x: usize, y: usize, class: Option<String>) -> String {
    let class_attr = if let Some(class_name) = class {
        format!("class=\"{}\"", class_name)
    } else {
        String::new()
    };

    format!(
        "<text x=\"{}\" y=\"{}\" {}>{}</text>",
        x, y, class_attr, text
    )
}
