use crate::clue::Clue;

pub fn place_clues(lines: Vec<String>) -> Vec<Clue> {
    sort(lines);
    vec![]
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
