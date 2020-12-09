use crate::common::*;
use itertools::Itertools;

#[derive(Default)]
pub struct Day {
    input: String,
}

impl AocDay for Day {
    fn parse_input(&mut self, input: &str) {
        self.input = input.into();
    }

    fn puzzle_00(&self) -> Option<AocPuzzleAnswer> {
        Some(find_max_seat_id(&self.input) as usize)
    }

    fn puzzle_01(&self) -> Option<AocPuzzleAnswer> {
        Some(find_my_seat_id(&self.input) as usize)
    }
}

enum Direction {
    Left,
    Right,
}

fn mid_point(left: u32, right: u32) -> u32 {
    (left + right) / 2
}

fn binary_step(direction: Direction, left: u32, right: u32) -> (u32, u32) {
    let mid = mid_point(left, right);
    match direction {
        Direction::Left => (left, mid),
        Direction::Right => (mid + 1, right),
    }
}

fn find_binary(left_char: char, right_char: char, s: &[char], left: u32, right: u32) -> u32 {
    match s {
        [] => {
            assert_eq!(left, right);
            left
        }
        [head, tail @ ..] => {
            let direction = if *head == left_char {
                Direction::Left
            } else if *head == right_char {
                Direction::Right
            } else {
                unreachable!()
            };

            let (new_left, new_right) = binary_step(direction, left, right);
            find_binary(left_char, right_char, tail, new_left, new_right)
        }
    }
}

fn find_row(s: &[char]) -> u32 {
    assert_eq!(s.len(), 7);
    find_binary('F', 'B', s, 0, 127)
}

fn find_column(s: &[char]) -> u32 {
    assert_eq!(s.len(), 3);
    find_binary('L', 'R', s, 0, 7)
}

fn find_seat(s: &[char]) -> (u32, u32) {
    assert_eq!(s.len(), 10);
    let (row_str, col_str) = s.split_at(7);
    (find_row(row_str), find_column(col_str))
}

fn seat_id(row: u32, col: u32) -> u32 {
    row * 8 + col
}

fn ticket_id_to_seat_id(ticket_id: &str) -> u32 {
    assert_eq!(ticket_id.len(), 10);
    let (row, col) = find_seat(&str_to_char_vec(ticket_id));
    seat_id(row, col)
}

fn find_max_seat_id(input: &str) -> u32 {
    input.lines().map(ticket_id_to_seat_id).max().unwrap()
}

fn find_my_seat_id(input: &str) -> u32 {
    let seat_ids = input
        .lines()
        .map(ticket_id_to_seat_id)
        .sorted()
        .collect::<Vec<u32>>();

    let (prev_seat, _) = seat_ids
        .windows(2)
        .map(|window| (window[0], window[1]))
        .find(|(prev, next)| next - prev == 2)
        .unwrap();

    let my_seat_id = prev_seat + 1;
    my_seat_id
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_seat() {
        assert_eq!(binary_step(Direction::Left, 0, 127), (0, 63));
        assert_eq!(binary_step(Direction::Right, 0, 63), (32, 63));
        assert_eq!(binary_step(Direction::Left, 32, 63), (32, 47));
        assert_eq!(binary_step(Direction::Right, 32, 47), (40, 47));
        assert_eq!(binary_step(Direction::Right, 40, 47), (44, 47));
        assert_eq!(binary_step(Direction::Left, 44, 47), (44, 45));
        assert_eq!(binary_step(Direction::Left, 44, 45), (44, 44));

        assert_eq!(find_row(&str_to_char_vec("FBFBBFF")), 44);
        assert_eq!(find_column(&str_to_char_vec("RLR")), 5);
        assert_eq!(super::find_seat(&str_to_char_vec("FBFBBFFRLR")), (44, 5));

        assert_eq!(seat_id(44, 5), 357);

        assert_eq!(ticket_id_to_seat_id("BFFFBBFRRR"), 567);
        assert_eq!(ticket_id_to_seat_id("FFFBBBFRRR"), 119);
        assert_eq!(ticket_id_to_seat_id("BBFFBBFRLL"), 820);
    }
}
