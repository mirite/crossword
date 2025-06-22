use std::{
    env,
    fmt::{Display, Formatter},
};

fn main() {
    let mut args = env::args();
    args.next();
    let hints: Vec<String> = args.collect();
    let mut clues: Vec<Clue> = Vec::new();
    for hint in hints {
        clues.push(read_clue(hint));
    }
    for clue in clues {
        println!("{}", clue);
    }
}

fn read_clue(hint: String) -> Clue {
    let mut state: State = State::InX;
    let mut clue: Clue = Clue {
        x: 0,
        y: 0,
        direction: Direction::Across,
        clue: String::new(),
    };

    let chars: Vec<char> = hint.chars().collect();
    let mut index = 0;
    while index < chars.len() {
        let c = chars[index];
        match state {
            State::InX => {
                if c.is_digit(10) {
                    clue.x = clue.x * 10 + c.to_digit(10).unwrap();
                } else {
                    state = State::InDirection;
                    continue; // Reprocess this character as the direction
                }
            }
            State::InY => {
                if c.is_digit(10) {
                    clue.y = clue.y * 10 + c.to_digit(10).unwrap();
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
                clue.clue.push(c);
            }
        }
        index += 1;
    }
    clue
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
    InX,
    InY,
    InDirection,
    InClue,
}

#[derive(Eq, PartialEq, Debug)]
pub struct Clue {
    x: u32,
    y: u32,
    direction: Direction,
    clue: String,
}
impl Display for Clue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({},{}) {}: {}",
            self.x, self.y, self.direction, self.clue
        )
    }
}
