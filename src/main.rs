#![feature(str_split_once)]

#[macro_use]
extern crate bitflags;

use crate::common::*;
use crate::setup::PuzzleDayInfo;

mod common;
mod day_00;
mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod setup;

fn main() {
    let mut days = setup::setup_puzzle_day_info_vec();

    for (i, day_info) in days.iter_mut().enumerate() {
        let (puzzle_00, puzzle_01) = get_answers(day_info);

        let print_puzzle_result = |result, i| match result {
            Some(x) => println!("\t{}. {}", i, x),
            None => println!("\t{}. UNIMPLEMENTED", i),
        };

        println!("day_{:02}", i);
        print_puzzle_result(puzzle_00, 0);
        print_puzzle_result(puzzle_01, 1);
    }
}

fn get_answers(day_info: &mut PuzzleDayInfo) -> (Option<AocPuzzleAnswer>, Option<AocPuzzleAnswer>) {
    let PuzzleDayInfo(day, input) = day_info;
    day.parse_input(input);
    let puzzle_00 = day.puzzle_00();
    let puzzle_01 = day.puzzle_01();
    (puzzle_00, puzzle_01)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn puzzle_answers_against_my_input() {
        let mut days = setup::setup_puzzle_day_info_vec();
        fn ensure_answers(
            day_info: &mut PuzzleDayInfo,
            expected_0: AocPuzzleAnswer,
            expected_1: AocPuzzleAnswer,
        ) {
            let (actual_0, actual_1) = get_answers(day_info);
            assert_eq!(actual_0, Some(expected_0));
            assert_eq!(actual_1, Some(expected_1));
        };

        ensure_answers(&mut days[0], 1006875, 165026160);
        ensure_answers(&mut days[1], 460, 251);
        ensure_answers(&mut days[2], 272, 3898725600);
        ensure_answers(&mut days[3], 256, 198);
        ensure_answers(&mut days[4], 864, 739);
        //ensure_answers(&mut days[5], 6782, ???); // can't seem to figure out what's wrong with my solution here...
        //ensure_answers(&mut days[6], ???, ???); // TODO
        //ensure_answers(&mut days[7], ???, ???); // TODO
        //ensure_answers(&mut days[8], ???, ???); // TODO
    }
}
