const CHAR_SPRING_OPERATIONAL: char = '.';
const CHAR_SPRING_DAMAGED: char = '#';
const CHAR_SPRING_UNKNOWN: char = '?';

#[derive(Clone)]
struct RecordState<'i> {
    group_size_current: usize,
    group_size_expected: &'i [usize],
    group_size_expected_cursor: usize,
    group_size_expected_cycle: usize,
}

impl<'i, 'j> RecordState<'i> {
    pub fn new(group_size_expected: &'i [usize], group_size_expected_cycle: usize) -> Self {
        Self {
            group_size_current: 0,
            group_size_expected,
            group_size_expected_cursor: 0,
            group_size_expected_cycle,
        }
    }
}

fn advance_state_operational(state_index: usize, states: &mut Vec<RecordState<'_>>) {
    let state = &mut states[state_index];
    if state.group_size_current != 0 {
        if state.group_size_current != state.group_size_expected[state.group_size_expected_cursor] {
            // group ends with invalid size
            states.remove(state_index);
        } else {
            // group ends with valid size
            state.group_size_current = 0;
            state.group_size_expected_cursor += 1;
            state.group_size_expected_cursor %= state.group_size_expected.len();
            if state.group_size_expected_cursor == 0 {
                // cycle detected
                state.group_size_expected_cycle -= 1;
            }
        }
    }
}

fn advance_state_damaged(state_index: usize, states: &mut Vec<RecordState<'_>>) {
    let state = &mut states[state_index];
    if state.group_size_current == state.group_size_expected[state.group_size_expected_cursor]
        || state.group_size_expected_cycle == 0
    {
        // group exceeds valid size
        states.remove(state_index);
    } else {
        // group has capacity for another damaged item
        state.group_size_current += 1;
    }
}

fn advance_state_unknown(state_index: usize, states: &mut Vec<RecordState<'_>>) {
    let state = &mut states[state_index];

    if state.group_size_expected_cycle == 0 {
        // => ? can only be .
        // do nothing
    } else if state.group_size_current != 0 {
        if state.group_size_current == state.group_size_expected[state.group_size_expected_cursor] {
            // found end group
            // => ? can only be .
            state.group_size_current = 0;
            state.group_size_expected_cursor += 1;
            state.group_size_expected_cursor %= state.group_size_expected.len();
            if state.group_size_expected_cursor == 0 {
                // cycle detected
                state.group_size_expected_cycle -= 1;
            }
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

pub fn get_sum_of_arrangements(input: &str, num_cycles: usize) -> usize {
    let mut sum: usize = 0;
    for line in input.lines() {
        let (report_line, damaged_spring_group_sizes_str) = line
            .rsplit_once(' ')
            .expect(&format!("could not find delimiter ' ' in line {}", line));
        let damaged_spring_group_sizes = damaged_spring_group_sizes_str
            .split(',')
            .map(|val| val.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let chars = report_line.chars().collect::<Vec<char>>();
        let mut chars_cursor = 0_usize;
        let mut chars_cursor_cycle = num_cycles;
        let mut states = vec![RecordState::new(&damaged_spring_group_sizes, num_cycles)];

        loop {
            // println!("{} {} ({:02}) ", report_line, chars[chars_cursor], chars_cursor);
            match chars[chars_cursor] {
                CHAR_SPRING_OPERATIONAL => {
                    // println!("{} {} ({:02}) > . (operational)", report_line, chars[chars_cursor], chars_cursor);
                    for i in (0..states.len()).rev() {
                        // println!(
                        //     "{} {} ({:02}) >> state gsc {} exp {} ({:02}) cy {}",
                        //     report_line,
                        //     chars[chars_cursor],
                        //     chars_cursor,
                        //     states[i].group_size_current,
                        //     states[i].group_size_expected[states[i].group_size_expected_cursor],
                        //     states[i].group_size_expected_cursor,
                        //     states[i].group_size_expected_cycle
                        // );
                        advance_state_operational(i, &mut states);
                    }
                }
                CHAR_SPRING_DAMAGED => {
                    // println!("{} {} ({:02}) > # (damaged)", report_line, chars[chars_cursor], chars_cursor);
                    for i in (0..states.len()).rev() {
                        // println!(
                        //     "{} {} ({:02}) >> state gsc {} exp {} ({:02}) cy {}",
                        //     report_line,
                        //     chars[chars_cursor],
                        //     chars_cursor,
                        //     states[i].group_size_current,
                        //     states[i].group_size_expected[states[i].group_size_expected_cursor],
                        //     states[i].group_size_expected_cursor,
                        //     states[i].group_size_expected_cycle
                        // );
                        advance_state_damaged(i, &mut states);
                    }
                }
                CHAR_SPRING_UNKNOWN => {
                    // println!("{} {} ({:02}) > ? (unknown)", report_line, chars[chars_cursor], chars_cursor);
                    for i in 0..states.len() {
                        // println!(
                        //     "{} {} ({:02}) >> state gsc {} exp {} ({:02}) cy {}",
                        //     report_line,
                        //     chars[chars_cursor],
                        //     chars_cursor,
                        //     states[i].group_size_current,
                        //     states[i].group_size_expected[states[i].group_size_expected_cursor],
                        //     states[i].group_size_expected_cursor,
                        //     states[i].group_size_expected_cycle
                        // );
                        advance_state_unknown(i, &mut states);
                    }
                }
                c => panic!("found unexpected char {}", c),
            }
            chars_cursor += 1;
            chars_cursor %= chars.len();
            if chars_cursor == 0 {
                // detected char cycle
                for i in 0..states.len() {
                    // println!(
                    //     "{} {} ({:02}) >> state gsc {} exp {} ({:02}) cy {} CHAR CYCLE ADVANCE",
                    //     report_line,
                    //     chars[chars_cursor],
                    //     chars_cursor,
                    //     states[i].group_size_current,
                    //     states[i].group_size_expected[states[i].group_size_expected_cursor],
                    //     states[i].group_size_expected_cursor,
                    //     states[i].group_size_expected_cycle
                    // );
                    advance_state_unknown(i, &mut states);
                }
                chars_cursor_cycle -= 1;
                if chars_cursor_cycle == 0 {
                    // chars have been depleted, remove invalid end states
                    states.retain_mut(|state| {
                        // if group_size_expected_cycle is 0, all group_sizes have already been verified
                        state.group_size_expected_cycle == 0
                        // otherwise (occurs when last char is 'damaged'), we need to verify if the last state
                        // is valid by:
                        // (a) ensuring that it is the very last group_size_expected_cycle
                        // (b) ensuring that the group size has an expected size
                        // (c) ensuring that the group_size_expected_cursor is at the very last position
                            || (state.group_size_expected_cycle == 1 &&
                                state.group_size_current
                                == state.group_size_expected[state.group_size_expected_cursor]
                                && state.group_size_expected_cursor
                                    == (state.group_size_expected.len() - 1))
                    });
                    break;
                }
            }
        }

        sum += states.len();
    }
    sum
}
