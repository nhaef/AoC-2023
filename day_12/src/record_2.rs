const CHAR_SPRING_OPERATIONAL: u8 = b'.';
const CHAR_SPRING_DAMAGED: u8 = b'#';
const CHAR_SPRING_UNKNOWN: u8 = b'?';

#[derive(Clone)]
struct RecordState<'i> {
    arrangements: usize,
    group_size_current: usize,
    group_sizes: &'i [usize],
    group_sizes_cursor: usize,
}

impl<'i> RecordState<'i> {
    pub fn new(group_sizes: &'i [usize]) -> Self {
        Self {
            arrangements: 1,
            group_size_current: 0,
            group_sizes,
            group_sizes_cursor: 0,
        }
    }
}

pub fn get_sum_of_arrangements(input: &str) -> usize {
    let mut sum: usize = 0;
    for line in input.lines() {
        let (report_line, damaged_spring_group_sizes_str) = line
            .rsplit_once(' ')
            .expect(&format!("could not find delimiter ' ' in line {}", line));
        let mut damaged_spring_group_sizes = damaged_spring_group_sizes_str
            .split(',')
            .map(|val| val.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
            .repeat(5);
        damaged_spring_group_sizes.push(0);

        let report_line = report_line.as_bytes();
        let mut report_line_repeated = report_line.to_owned();
        for _ in 0..4 {
            report_line_repeated.push(CHAR_SPRING_UNKNOWN);
            report_line_repeated.extend(report_line);
        }
        report_line_repeated.push(CHAR_SPRING_OPERATIONAL);

        // println!("report_line: {} (original)", std::str::from_utf8(report_line).unwrap());
        // println!("report_line: {}", std::str::from_utf8(&report_line_repeated).unwrap());
        // println!("group_sizes: {:?}", damaged_spring_group_sizes);

        let initial_state = RecordState::new(&damaged_spring_group_sizes);
        let mut states = vec![initial_state];
        for char in report_line_repeated {
            match char {
                CHAR_SPRING_OPERATIONAL => {
                    states.retain_mut(|state| {
                        // handle char
                        if state.group_size_current != 0 {
                            if state.group_size_current
                                != state.group_sizes[state.group_sizes_cursor]
                            {
                                // group ends with invalid size
                                return false;
                            }
                            // group ends with valid size
                            state.group_sizes_cursor += 1;
                            state.group_size_current = 0;
                        }
                        true
                    });
                }
                CHAR_SPRING_DAMAGED => {
                    states.retain_mut(|state| {
                        // handle char
                        if state.group_size_current == state.group_sizes[state.group_sizes_cursor] {
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
                            if state.group_size_current
                                == state.group_sizes[state.group_sizes_cursor]
                            {
                                // found end group
                                // => ? can only be .
                                state.group_sizes_cursor += 1;
                                state.group_size_current = 0;
                            } else {
                                // expected group size not yet reached
                                // => ? can only be #
                                state.group_size_current += 1;
                            }
                        } else {
                            // current group size is 0
                            // => ? can be either . or #
                            let new_state = state.clone();
                            state.group_size_current = 1;
                            states.push(new_state);
                        }
                    }
                }
                c => panic!("found unexpected char {}", c),
            }
            // remove duplicate states
            let mut visited_states: std::collections::HashMap<usize, RecordState<'_>> =
                std::collections::HashMap::new();
            for state in states.into_iter() {
                let ident = state.group_size_current + state.group_sizes_cursor * 1_000;
                if let Some(other_state) = visited_states.get_mut(&ident) {
                    other_state.arrangements += state.arrangements;
                } else {
                    visited_states.insert(ident, state);
                }
            }
            states = visited_states.into_values().collect();
        }
        let arrangements = states
            .into_iter()
            .filter(|s| s.group_sizes[s.group_sizes_cursor] == 0)
            .fold(0, |acc, s| acc + s.arrangements);
        sum += arrangements;
    }
    sum
}
