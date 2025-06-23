use std::fmt::{Display, Formatter};

#[derive(Eq, PartialEq, Debug)]
pub struct Clue {
    pub x: u32,
    pub y: u32,
    pub direction: Direction,
    pub clue: String,
    pub answer: String,
}
impl Display for Clue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({},{}) {}: {}\n\t{}",
            self.x, self.y, self.direction, self.clue, self.answer
        )
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
