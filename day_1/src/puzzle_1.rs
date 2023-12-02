pub fn get_calibration_value_sum(input: &str) -> u32 {
    assert!(input.is_ascii());
    input.lines().map(|line| get_calibration_value(line)).sum()
}

pub fn get_calibration_value(line: &str) -> u32 {
    let digits: Vec<u32> = line
        .chars()
        .filter_map(|c| try_convert_ascii_digit_to_u32(c))
        .collect();
    assert!(
        digits.len() > 0,
        "Could not find any digits in line {}",
        line
    );
    let first_digit = digits.first().unwrap();
    let last_digit = digits.last().unwrap();
    first_digit * 10 + last_digit
}

pub fn try_convert_ascii_digit_to_u32(c: char) -> Option<u32> {
    if c.is_ascii_digit() {
        Some((c as u32) - 48)
    } else {
        None
    }
}
