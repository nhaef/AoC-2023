use crate::*;

pub fn get_calibration_value_sum(input: &str) -> u32 {
    assert!(input.is_ascii());
    input.lines().map(|line| get_calibration_value(line)).sum()
}

fn get_calibration_value(line: &str) -> u32 {
    let first_digit = find_first_digit(line);
    let last_digit = find_last_digit(line);

    first_digit * 10 + last_digit
}

fn find_first_digit(line: &str) -> u32 {
    for i in 0..line.len() {
        if let Some(digit) = puzzle_1::try_convert_ascii_digit_to_u32(line.as_bytes()[i] as char) {
            return digit;
        }
        test_words_from_end!(line, i, 3, [
            ["one", 1],
            ["two", 2],
            ["six", 6]
        ]);
        test_words_from_end!(line, i, 4, [
            ["four", 4],
            ["five", 5],
            ["nine", 9]
        ]);
        test_words_from_end!(line, i, 5, [
            ["three", 3],
            ["seven", 7],
            ["eight", 8]
        ]);
    }
    panic!("Could not find first digit in line {}", line);
}

fn find_last_digit(line: &str) -> u32 {
    for i in (0..line.len()).rev() {
        if let Some(digit) = puzzle_1::try_convert_ascii_digit_to_u32(line.as_bytes()[i] as char) {
            return digit;
        }
        test_words_from_start!(line, i, 3, [
            ["one", 1],
            ["two", 2],
            ["six", 6]
        ]);
        test_words_from_start!(line, i, 4, [
            ["four", 4],
            ["five", 5],
            ["nine", 9]
        ]);
        test_words_from_start!(line, i, 5, [
            ["three", 3],
            ["seven", 7],
            ["eight", 8]
        ]);
    }
    panic!("Could not find last digit in line {}", line);
}

#[macro_export]
macro_rules! test_words_from_end {
    ($line:expr, $end_pos:expr, $word_len:expr, [$([$w:expr, $v:expr]),+]) => {
        if let Some(word_start) = $end_pos.checked_sub($word_len - 1) {
            match &$line[word_start..=$end_pos] {
                $(
                    $w => return $v,
                )+
                _ => (),
            }
        }
    };
}

#[macro_export]
macro_rules! test_words_from_start {
    ($line:expr, $start_pos:expr, $word_len:expr, [$([$w:expr, $v:expr]),+]) => {
        let word_end = $start_pos + $word_len - 1;
        if word_end < $line.len() {
            match &$line[$start_pos..=word_end] {
                $(
                    $w => return $v,
                )+
                _ => (),
            }
        }
    };
}