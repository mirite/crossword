use std::fmt::{Display, Formatter};

use crate::placement::BaseClue;

#[derive(Eq, PartialEq, Debug)]
pub struct Clue {
    pub x: u8,
    pub y: u8,
    pub direction: Direction,
    pub base: BaseClue,
    pub number: u8,
}
impl Display for Clue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({},{}) {}: {}\n\t{}",
            self.x, self.y, self.direction, self.base.clue, self.base.answer
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
