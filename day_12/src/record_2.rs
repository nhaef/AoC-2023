use std::str::from_utf8;

const CHAR_SPRING_OPERATIONAL: u8 = b'.';
const CHAR_SPRING_DAMAGED: u8 = b'#';
const CHAR_SPRING_UNKNOWN: u8 = b'?';

#[derive(Clone)]
struct RecordState<'i> {
    arrangements: usize,
    current_group_size: usize,
    group_sizes: &'i [u8],
    group_sizes_cursor: usize,
}

impl<'i> RecordState<'i> {
    pub fn new(group_sizes: &'i [u8]) -> Self {
        Self {
            arrangements: 1,
            current_group_size: 0,
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
        let damaged_spring_group_sizes = damaged_spring_group_sizes_str
            .split(',')
            .map(|val| val.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        
        let report_line = report_line.as_bytes();
        let mut report_line_repeated = report_line.to_owned();
        for _ in 0..4 {
            report_line_repeated.push(CHAR_SPRING_UNKNOWN);
            report_line_repeated.extend(report_line);
        }

        println!("report_line: {} (original)", from_utf8(report_line).unwrap());
        println!("report_line: {}", from_utf8(&report_line_repeated).unwrap());

        let mut states = vec![RecordState::new(&report_line_repeated)];
        // for char in report_line.chars() {
        //     match char {
        //         CHAR_SPRING_OPERATIONAL => {
        //             states.retain_mut(|state| {
        //                 if state.group_size_current != 0 {
        //                     if state.group_size_current != *state.group_size_expected {
        //                         // group ends with invalid size
        //                         return false;
        //                     }
        //                     // group ends with valid size
        //                     state.group_size_expected =
        //                         match state.group_size_expected_iterator.next() {
        //                             Some(v) => v,
        //                             None => &0,
        //                         };
        //                     state.group_size_current = 0;
        //                 }
        //                 true
        //             });
        //         }
        //         CHAR_SPRING_DAMAGED => {
        //             states.retain_mut(|state| {
        //                 if state.group_size_current == *state.group_size_expected {
        //                     // group exceeds valid size
        //                     return false;
        //                 }
        //                 // group has capacity for another damaged item
        //                 state.group_size_current += 1;
        //                 true
        //             });
        //         }
        //         CHAR_SPRING_UNKNOWN => {
        //             for i in 0..states.len() {
        //                 let state = &mut states[i];
        //                 if state.group_size_current != 0 {
        //                     if state.group_size_current == *state.group_size_expected {
        //                         // found end group
        //                         // => ? can only be .
        //                         state.group_size_expected =
        //                             match state.group_size_expected_iterator.next() {
        //                                 Some(v) => v,
        //                                 None => &0,
        //                             };
        //                         state.group_size_current = 0;
        //                     } else {
        //                         // group_size_expected not reached
        //                         // => ? can only be #
        //                         state.group_size_current += 1;
        //                     }
        //                 } else {
        //                     // current group size is 0
        //                     // => ? can be either . or #
        //                     let state_operational = state.clone();
        //                     let state_damaged = state;
        //                     state_damaged.group_size_current = 1;
        //                     states.push(state_operational);
        //                 }
        //             }
        //         }
        //         c => panic!("found unexpected char {}", c),
        //     }
        // }
        // states.retain_mut(|state| {
        //     state.group_size_current == *state.group_size_expected
        //         && state.group_size_expected_iterator.next().is_none()
        // });
        sum += states.len();
    }
    sum
}
