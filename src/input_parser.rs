use std::error;

use crate::clue::{Clue, Direction};
#[derive(Debug, thiserror::Error)]
pub enum ClueParseError {
    #[error("Invalid direction character: {0}")]
    InvalidDirection(char),
    #[error("Missing expected character")]
    MissingCharacter,
    #[error("Failed to parse number: {0}")]
    NumberParseError(#[from] std::num::ParseIntError),
    #[error("Unexpected end of input")]
    UnexpectedEndOfInput,
}
pub fn read_clue(hint: String) -> Clue {
    let mut state: State = State::InX;
    let mut clue: Clue = Clue {
        x: 0,
        y: 0,
        number: 1,
        direction: Direction::Across,
        clue: String::new(),
        answer: String::new(),
    };

    let chars: Vec<char> = hint.chars().collect();
    let mut index = 0;
    while index < chars.len() {
        let c = chars[index];
        match state {
            State::InX => {
                if c.is_digit(10) {
                    clue.x = clue.x * 10 + c.to_digit(10).unwrap() as u8;
                } else {
                    state = State::InDirection;
                    continue; // Reprocess this character as the direction
                }
            }
            State::InY => {
                if c.is_digit(10) {
                    clue.y = clue.y * 10 + c.to_digit(10).unwrap() as u8;
                } else {
                    state = State::InClue;
                    continue; // Reprocess this character as the direction
                }
            }
            State::InDirection => {
                clue.direction = match c {
                    'D' => Direction::Down,
                    'A' => Direction::Across,
                    _ => panic!("Invalid direction {}", c),
                };
                state = State::InY;
            }
            State::InClue => {
                if c == ';' {
                    state = State::InAnswer;
                } else {
                    clue.clue.push(c);
                }
            }
            State::InAnswer => clue.answer.push(c),
        }
        index += 1;
    }
    clue
}

#[derive(Eq, PartialEq, Debug)]
pub enum State {
    InX,
    InY,
    InDirection,
    InClue,
    InAnswer,
}
