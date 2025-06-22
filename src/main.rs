use std::{
    env,
    fmt::{Display, Formatter},
};

fn main() {
    let mut args = env::args();
    args.next();
    let hints: Vec<String> = args.collect();

    for hint in hints {
        let mut state: State = State::InNumber;
        let mut clue: Clue = Clue {
            number: 0,
            direction: Direction::Across,
            clue: String::new(),
        };

        let chars: Vec<char> = hint.chars().collect();
        for c in chars {
            print!("Processing {}", c);
            state = match state {
                State::InNumber => {
                    if c.is_digit(10) {
                        clue.number = (clue.number * 10) + c.to_digit(10).unwrap();
                        State::InNumber
                    } else {
                        State::InDirection
                    }
                }
                State::InDirection => {
                    clue.direction = match c {
                        'D' => Direction::Down,
                        'A' => Direction::Across,
                        _ => panic!("Invalid direction {}", c),
                    };
                    State::InClue
                }
                State::InClue => {
                    clue.clue.push(c);
                    State::InClue
                }
            };
        }
        println!(
            "Result: Number {}, Direction {}, Clue {}",
            clue.number, clue.direction, clue.clue
        );
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum Direction {
    Down,
    Across,
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::Down => "Down",
                Direction::Across => "Across",
            }
        )
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum State {
    InNumber,
    InDirection,
    InClue,
}

#[derive(Eq, PartialEq, Debug)]
pub struct Clue {
    number: u32,
    direction: Direction,
    clue: String,
}
