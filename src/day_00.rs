use crate::common::*;

#[derive(Default)]
pub struct Day {
    numbers: Vec<u32>,
}

impl AocDay for Day {
    fn parse_input(&mut self, input: &str) {
        self.numbers = input
            .split_whitespace()
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();
    }

    fn puzzle_00(&self) -> Option<AocPuzzleAnswer> {
        let (a, b) = find_2_numbers_sum_2020(&self.numbers);
        assert_eq!(a + b, 2020);
        Some((a * b) as usize)
    }

    fn puzzle_01(&self) -> Option<AocPuzzleAnswer> {
        let (a, b, c) = find_3_numbers_sum_2020(&self.numbers);
        assert_eq!(a + b + c, 2020);
        Some((a * b * c) as usize)
    }
}

fn find_2_numbers_sum_2020(numbers: &[u32]) -> (u32, u32) {
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if i == j {
                continue;
            }

            let a = numbers[i];
            let b = numbers[j];
            if a + b == 2020 {
                return (a, b);
            }
        }
    }

    panic!("No numbers add up to 2020!");
}

fn find_3_numbers_sum_2020(numbers: &[u32]) -> (u32, u32, u32) {
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if i == j {
                continue;
            }

            for k in 0..numbers.len() {
                if k == i || k == j {
                    continue;
                }

                let a = numbers[i];
                let b = numbers[j];
                let c = numbers[k];
                if a + b + c == 2020 {
                    return (a, b, c);
                }
            }
        }
    }

    panic!("No three numbers add up to 2020!");
}
