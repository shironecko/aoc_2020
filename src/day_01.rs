use crate::common::*;
use std::convert::{TryFrom, TryInto};

#[derive(Default)]
pub struct Day {
    entries: Vec<Entry>,
}

impl AocDay for Day {
    fn parse_input(&mut self, input: &str) {
        self.entries = input
            .lines()
            .filter_map(|line| line.try_into().ok())
            .collect();
    }

    fn puzzle_00(&self) -> Option<AocPuzzleAnswer> {
        Some(count_valid_passwords_v1(&self.entries))
    }

    fn puzzle_01(&self) -> Option<AocPuzzleAnswer> {
        Some(count_valid_passwords_v2(&self.entries))
    }
}

struct Entry {
    min: usize,
    max: usize,
    required_char: char,
    password: String,
}

impl TryFrom<&str> for Entry {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (head, tail) = value
            .split_once('-')
            .ok_or_else(|| Self::Error::msg("min"))?;
        let min = head.parse::<usize>()?;

        let (head, tail) = tail
            .split_once(' ')
            .ok_or_else(|| Self::Error::msg("max"))?;
        let max = head.parse::<usize>()?;

        let (head, tail) = tail
            .split_once(':')
            .ok_or_else(|| Self::Error::msg("required_char split"))?;
        let required_char = head
            .trim()
            .chars()
            .nth(0)
            .ok_or_else(|| Self::Error::msg("required_char"))?;
        let password = tail.trim().to_owned();

        Ok(Self {
            min,
            max,
            required_char,
            password,
        })
    }
}

fn count_valid_passwords_v1(entries: &[Entry]) -> usize {
    entries
        .iter()
        .filter(|&entry| {
            let count = entry
                .password
                .chars()
                .filter(|&c| c == entry.required_char)
                .count();
            (entry.min..=entry.max).contains(&count)
        })
        .count()
}

fn count_valid_passwords_v2(entries: &[Entry]) -> usize {
    entries
        .iter()
        .filter(|&entry| {
            let first_match =
                entry.password.chars().nth(entry.min - 1).unwrap() == entry.required_char;
            let second_match =
                entry.password.chars().nth(entry.max - 1).unwrap() == entry.required_char;

            (first_match || second_match) && !(first_match && second_match)
        })
        .count()
}
