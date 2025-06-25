use crate::clue::{Clue, Direction};

pub fn read_clue(hint: String) -> Clue {
    let mut state: State = State::InX;
    let mut clue: Clue = Clue {
        x: 0,
        y: 0,
        number: 0,
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
                    state = State::InY;
                }
            }
            State::InY => {
                if c.is_digit(10) {
                    clue.y = clue.y * 10 + c.to_digit(10).unwrap() as u8;
                } else {
                    state = State::InDirection;
                    continue; // Reprocess this character as the direction
                }
            }
            State::InDirection => {
                clue.direction = match c {
                    'D' => Direction::Down,
                    'A' => Direction::Across,
                    _ => panic!("Invalid direction {}", c),
                };
                state = State::InNumber;
            }
            State::InNumber => {
                if c.is_digit(10) {
                    clue.number = clue.number * 10 + c.to_digit(10).unwrap() as u8;
                } else {
                    state = State::InClue;
                    continue; // Reprocess this character as the start of the clue
                }
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
    InNumber,
    InAnswer,
}
