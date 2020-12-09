use crate::common::*;
use itertools::iterate;
use std::ops::Add;

#[derive(Default)]
pub struct Day {
    map: Map,
}

impl AocDay for Day {
    fn parse_input(&mut self, input: &str) {
        self.map = input.into();
    }

    fn puzzle_00(&self) -> Option<AocPuzzleAnswer> {
        let direction = Vec2(3, 1);
        Some(count_trees(&self.map, direction))
    }

    fn puzzle_01(&self) -> Option<AocPuzzleAnswer> {
        let directions = [Vec2(1, 1), Vec2(3, 1), Vec2(5, 1), Vec2(7, 1), Vec2(1, 2)];
        Some(multiply_trees_on_slopes(&self.map, &directions))
    }
}

#[derive(Copy, Clone)]
pub struct Vec2(usize, usize);

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Default)]
struct Map {
    rows: Vec<Vec<bool>>,
}

impl From<&str> for Map {
    fn from(s: &str) -> Self {
        let rows = s
            .lines()
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|c| match c {
                        '.' => false,
                        '#' => true,
                        _ => unreachable!("Unexpected symbol in string!"),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Map { rows }
    }
}

impl Map {
    fn w(&self) -> usize {
        self.rows
            .first()
            .expect("Can't get the width of an empty map!")
            .len()
    }

    fn h(&self) -> usize {
        self.rows.len()
    }

    fn in_bounds(&self, pos: Vec2) -> bool {
        // world wraps around itself on X plane
        pos.1 < self.h()
    }

    fn wrap(&self, pos: Vec2) -> Vec2 {
        Vec2(pos.0 % self.w(), pos.1)
    }

    fn at(&self, pos: Vec2) -> bool {
        self.rows[pos.1][pos.0]
    }
}

fn count_trees(map: &Map, step: Vec2) -> usize {
    iterate(Vec2(0, 0), |&x| map.wrap(x + step))
        .take_while(|&x| map.in_bounds(x))
        .filter(|&x| map.at(x))
        .count()
}

fn multiply_trees_on_slopes(map: &Map, steps: &[Vec2]) -> usize {
    steps.iter().map(|&step| count_trees(map, step)).product()
}
