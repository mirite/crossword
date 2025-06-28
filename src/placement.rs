use crate::clue::Clue;

#[derive(Clone, Copy)]
union SquareValue {
    char: char,
    blank: bool,
}

#[derive(Eq, PartialEq, Debug)]
pub struct BaseClue {
    pub clue: String,
    pub answer: String,
}

pub fn place_clues(lines: Vec<String>) -> Vec<Clue> {
    let sorted = sort(lines);
    let size: usize = sorted[0].len();
    let grid: Vec<Vec<Option<SquareValue>>> = vec![vec![None; size]; size];
    let mut clues: Vec<Clue> = vec![];
    clues
}

fn sort(mut strings: Vec<String>) -> Vec<String> {
    strings.sort_by(|a, b| b.len().cmp(&a.len()));
    strings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orders_correctly() {
        let mut strings = vec![
            String::from("a"),
            String::from("aa"),
            String::from("ab"),
            String::from("bbb"),
        ];
        strings = sort(strings);
        let expected = vec![
            String::from("bbb"),
            String::from("aa"),
            String::from("ab"),
            String::from("a"),
        ];
        assert_eq!(strings, expected);
    }
}
