use crate::placement::BaseClue;
pub fn sort(mut strings: Vec<BaseClue>) -> Vec<BaseClue> {
    strings.sort_by(|a, b| b.answer.len().cmp(&a.answer.len()));
    strings
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn orders_correctly() {
        let input = vec![
            BaseClue {
                clue: "c".into(),
                answer: "a".into(),
            },
            BaseClue {
                clue: "b".into(),
                answer: "aa".into(),
            },
            BaseClue {
                clue: "a".into(),
                answer: "ab".into(),
            },
            BaseClue {
                clue: "d".into(),
                answer: "bbb".into(),
            },
        ];

        let sorted = sort(input.clone());
        let expected = vec![
            BaseClue {
                clue: "d".into(),
                answer: "bbb".into(),
            },
            BaseClue {
                clue: "b".into(),
                answer: "aa".into(),
            },
            BaseClue {
                clue: "a".into(),
                answer: "ab".into(),
            },
            BaseClue {
                clue: "c".into(),
                answer: "a".into(),
            },
        ];

        assert_eq!(sorted, expected);
    }
}
