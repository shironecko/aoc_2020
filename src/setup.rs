use crate::common::AocDay;
use crate::{day_00, day_01, day_02, day_03, day_04, day_05, day_06, day_07, day_08};

pub struct PuzzleDayInfo(pub Box<dyn AocDay>, pub &'static str);

pub fn setup_puzzle_day_info_vec() -> Vec<PuzzleDayInfo> {
    vec![
        PuzzleDayInfo(
            Box::new(day_00::Day::default()),
            include_str!("../inputs/day_00.txt"),
        ),
        PuzzleDayInfo(
            Box::new(day_01::Day::default()),
            include_str!("../inputs/day_01.txt"),
        ),
        PuzzleDayInfo(
            Box::new(day_02::Day::default()),
            include_str!("../inputs/day_02.txt"),
        ),
        PuzzleDayInfo(
            Box::new(day_03::Day::default()),
            include_str!("../inputs/day_03.txt"),
        ),
        PuzzleDayInfo(
            Box::new(day_04::Day::default()),
            include_str!("../inputs/day_04.txt"),
        ),
        PuzzleDayInfo(
            Box::new(day_05::Day::default()),
            include_str!("../inputs/day_05.txt"),
        ),
        PuzzleDayInfo(
            Box::new(day_06::Day::default()),
            include_str!("../inputs/day_06.txt"),
        ),
        PuzzleDayInfo(
            Box::new(day_07::Day::default()),
            include_str!("../inputs/day_07.txt"),
        ),
        PuzzleDayInfo(
            Box::new(day_08::Day::default()),
            include_str!("../inputs/day_08.txt"),
        ),
    ]
}
