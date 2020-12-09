use crate::common::*;

#[derive(Default)]
pub struct Day {
    passport_data_groups: Vec<Vec<String>>,
}

impl AocDay for Day {
    fn parse_input(&mut self, input: &str) {
        self.passport_data_groups = group_by_empty_lines(input);
    }

    fn puzzle_00(&self) -> Option<AocPuzzleAnswer> {
        Some(count_valid_passports(&self.passport_data_groups))
    }

    fn puzzle_01(&self) -> Option<AocPuzzleAnswer> {
        Some(count_valid_passports_ex(&self.passport_data_groups))
    }
}

bitflags! {
    struct Fields: u32 {
        const BYR = 0b0000001;
        const IYR = 0b0000010;
        const EYR = 0b0000100;
        const HGT = 0b0001000;
        const HCL = 0b0010000;
        const ECL = 0b0100000;
        const PID = 0b1000000;
        const ALL = 0b1111111;
    }
}

fn str_to_field(string: &str) -> Option<Fields> {
    match string.split(':').next().unwrap() {
        "byr" => Some(Fields::BYR),
        "iyr" => Some(Fields::IYR),
        "eyr" => Some(Fields::EYR),
        "hgt" => Some(Fields::HGT),
        "hcl" => Some(Fields::HCL),
        "ecl" => Some(Fields::ECL),
        "pid" => Some(Fields::PID),
        _ => None,
    }
}

fn str_to_field_u32(string: &str) -> u32 {
    if let Some(field) = str_to_field(string) {
        field.bits
    } else {
        0
    }
}

fn is_valid_year(string: &str, min: u16, max: u16) -> bool {
    if let Ok(year) = string.parse::<u16>() {
        year >= min && year <= max
    } else {
        false
    }
}

fn is_valid_byr(string: &str) -> bool {
    is_valid_year(string, 1920, 2002)
}

fn is_valid_iyr(string: &str) -> bool {
    is_valid_year(string, 2010, 2020)
}

fn is_valid_eyr(string: &str) -> bool {
    is_valid_year(string, 2020, 2030)
}

fn is_valid_height(string: &str) -> bool {
    let parts = string.split_at(string.len() - 2);
    if let Ok(height) = parts.0.parse::<u8>() {
        match parts.1 {
            "cm" => height >= 150 && height <= 193,
            "in" => height >= 59 && height <= 76,
            _ => false,
        }
    } else {
        false
    }
}

fn is_valid_hair_color(string: &str) -> bool {
    if let Some('#') = string.chars().nth(0) {
        let rest: String = string.chars().skip(1).collect();
        rest.len() == 6
            && rest
                .chars()
                .all(|c| c.is_ascii_digit() || (c >= 'a' && c <= 'f'))
    } else {
        false
    }
}

fn is_valid_eye_color(string: &str) -> bool {
    const VALID_COLORS: &[&str] = &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    VALID_COLORS.contains(&string)
}

fn is_valid_passport_number(string: &str) -> bool {
    string.len() == 9 && string.chars().all(|c| c.is_ascii_digit())
}

fn str_to_field_with_validation(string: &str) -> Option<u32> {
    let data = string.split(':').nth(1).unwrap();
    let valid = if let Some(field) = str_to_field(string) {
        match field {
            Fields::BYR => is_valid_byr(data),
            Fields::IYR => is_valid_iyr(data),
            Fields::EYR => is_valid_eyr(data),
            Fields::HGT => is_valid_height(data),
            Fields::HCL => is_valid_hair_color(data),
            Fields::ECL => is_valid_eye_color(data),
            Fields::PID => is_valid_passport_number(data),
            _ => unreachable!(),
        }
    } else {
        true
    };

    if valid {
        Some(str_to_field_u32(string))
    } else {
        None
    }
}

fn count_valid_passports(data_groups: &Vec<Vec<String>>) -> usize {
    data_groups
        .iter()
        .map(|group| group.iter().flat_map(|line| line.split(' ')))
        .map(|group| group.map(str_to_field_u32).fold(0, |acc, x| acc | x))
        .filter(|&x| x & Fields::ALL.bits == Fields::ALL.bits)
        .count()
}

fn count_valid_passports_ex(data_groups: &Vec<Vec<String>>) -> usize {
    data_groups
        .iter()
        .map(|group| group.iter().flat_map(|line| line.split(' ')))
        .map(|group| {
            group
                .filter_map(str_to_field_with_validation)
                .fold(0, |acc, x| acc | x)
        })
        .filter(|&x| x & Fields::ALL.bits == Fields::ALL.bits)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_valid_passports_ex_example() {
        let data_groups = group_by_empty_lines(EXAMPLE_INPUT);
        let answer = count_valid_passports_ex(&data_groups);
        assert_eq!(answer, 2);
    }

    #[test]
    fn passport_field_validation() {
        assert!(is_valid_byr("2002"));
        assert!(!is_valid_byr("2003"));

        assert!(is_valid_height("60in"));
        assert!(is_valid_height("190cm"));
        assert!(!is_valid_height("190in"));
        assert!(!is_valid_height("190"));

        assert!(is_valid_hair_color("#123abc"));
        assert!(!is_valid_hair_color("#123abz"));
        assert!(!is_valid_hair_color("123abc"));

        assert!(is_valid_eye_color("brn"));
        assert!(!is_valid_eye_color("wat"));

        assert!(is_valid_passport_number("000000001"));
        assert!(!is_valid_passport_number("0123456789"));
    }

    const EXAMPLE_INPUT: &'static str = "\
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
";
}
