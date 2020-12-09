use itertools::Itertools;

pub type AocPuzzleAnswer = usize;

pub trait AocDay {
    fn parse_input(&mut self, input: &str);
    fn puzzle_00(&self) -> Option<AocPuzzleAnswer>;
    fn puzzle_01(&self) -> Option<AocPuzzleAnswer>;
}

pub fn group_by_empty_lines(input: &str) -> Vec<Vec<String>> {
    input
        .lines()
        .group_by(|line| line.trim().is_empty())
        .into_iter()
        .filter(|(key, _)| !key)
        .map(|(_, lines)| lines.map(String::from).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

pub fn str_to_char_vec(s: &str) -> Vec<char> {
    s.chars().collect()
}
