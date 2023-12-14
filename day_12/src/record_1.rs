const CHAR_SPRING_OPERATIONAL: char = '.';
const CHAR_SPRING_DAMAGED: char = '#';
const CHAR_SPRING_UNKNOWN: char = '?';

#[derive(Clone)]
struct RecordState<'i> {
    group_size_current: usize,
    group_size_expected: &'i usize,
    group_size_expected_iterator: std::slice::Iter<'i, usize>,
}

impl<'i> RecordState<'i> {
    pub fn new(mut group_size_expected_iterator: std::slice::Iter<'i, usize>) -> Self {
        let first_expected_size = group_size_expected_iterator.next().unwrap();
        Self {
            group_size_current: 0,
            group_size_expected: first_expected_size,
            group_size_expected_iterator,
        }
    }
}

pub fn get_sum_of_arrangements(input: &str) -> usize {
    let mut sum: usize = 0;
    for line in input.lines() {
        let (report_line, damaged_spring_group_sizes_str) = line
            .rsplit_once(' ')
            .expect(&format!("could not find delimiter ' ' in line {}", line));
        let damaged_spring_group_sizes = damaged_spring_group_sizes_str
            .split(',')
            .map(|val| val.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let mut states = vec![RecordState::new(damaged_spring_group_sizes.iter())];
        for char in report_line.chars() {
            match char {
                CHAR_SPRING_OPERATIONAL => {
                    states.retain_mut(|state| {
                        if state.group_size_current != 0 {
                            if state.group_size_current != *state.group_size_expected {
                                // group ends with invalid size
                                return false;
                            }
                            // group ends with valid size
                            state.group_size_expected =
                                match state.group_size_expected_iterator.next() {
                                    Some(v) => v,
                                    None => &0,
                                };
                            state.group_size_current = 0;
                        }
                        true
                    });
                }
                CHAR_SPRING_DAMAGED => {
                    states.retain_mut(|state| {
                        if state.group_size_current == *state.group_size_expected {
                            // group exceeds valid size
                            return false;
                        }
                        // group has capacity for another damaged item
                        state.group_size_current += 1;
                        true
                    });
                }
                CHAR_SPRING_UNKNOWN => {
                    for i in 0..states.len() {
                        let state = &mut states[i];
                        if state.group_size_current != 0 {
                            if state.group_size_current == *state.group_size_expected {
                                // found end group
                                // => ? can only be .
                                state.group_size_expected =
                                    match state.group_size_expected_iterator.next() {
                                        Some(v) => v,
                                        None => &0,
                                    };
                                state.group_size_current = 0;
                            } else {
                                // group_size_expected not reached
                                // => ? can only be #
                                state.group_size_current += 1;
                            }
                        } else {
                            // current group size is 0
                            // => ? can be either . or #
                            let state_operational = state.clone();
                            let state_damaged = state;
                            state_damaged.group_size_current = 1;
                            states.push(state_operational);
                        }
                    }
                }
                c => panic!("found unexpected char {}", c),
            }
        }
        states.retain_mut(|state| {
            state.group_size_current == *state.group_size_expected
                && state.group_size_expected_iterator.next().is_none()
        });
        sum += states.len();
    }
    sum
}
