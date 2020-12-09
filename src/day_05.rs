use crate::common::*;
use std::collections::HashSet;

#[derive(Default)]
pub struct Day {
    answer_groups: Vec<Vec<String>>,
}

impl AocDay for Day {
    fn parse_input(&mut self, input: &str) {
        self.answer_groups = group_by_empty_lines(input);
    }

    fn puzzle_00(&self) -> Option<AocPuzzleAnswer> {
        Some(count_unique_answers(&self.answer_groups))
    }

    fn puzzle_01(&self) -> Option<AocPuzzleAnswer> {
        Some(count_unanimous_answers(&self.answer_groups))
    }
}

fn count_unique_answers(answer_groups: &Vec<Vec<String>>) -> usize {
    answer_groups
        .iter()
        .map(|lines| {
            lines
                .into_iter()
                .fold(String::new(), |acc, s| acc + s.trim())
        })
        .map(|s| s.chars().collect::<HashSet<_>>().len())
        .sum()
}

fn count_unanimous_answers(answer_groups: &Vec<Vec<String>>) -> usize {
    answer_groups
        .iter()
        .map(|lines| {
            lines
                .into_iter()
                .map(|s| s.trim().chars().collect::<HashSet<_>>())
        })
        .map(|sets| {
            sets.fold(('a'..'z').collect::<HashSet<_>>(), |a, b| {
                a.intersection(&b).cloned().collect()
            })
        })
        .map(|set| set.len())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unanimous_answers() {
        assert_eq!(count_unanimous_answers(&group_by_empty_lines("a\nb\nc")), 0);
        let input = "\
    ab
    ac";
        assert_eq!(count_unanimous_answers(&group_by_empty_lines(input)), 1);
        let input = "\
    abc

    a
    b
    c

    ab
    ac

    a
    a
    a
    a

    b";
        assert_eq!(count_unanimous_answers(&group_by_empty_lines(input)), 6);
    }
}
