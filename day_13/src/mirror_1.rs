pub fn summarize_all_notes(input: &str) -> usize {
    let mut sum = 0;
    let mut current_pattern = vec![];
    'line_loop: for line in input.lines().into_iter().chain([""]) {
        if !line.is_empty() {
            current_pattern.push(line);
            continue;
        }

        // try to find horizontal reflection line
        let num_rows = current_pattern.len();
        for i in 1..num_rows {
            for j in 1.. {
                let (up_index, up_index_overflow) = i.overflowing_sub(j);
                let down_index = i + j - 1;
                if up_index_overflow || down_index == num_rows {
                    // reached end of valid reflection
                    sum += 100 * i;
                    current_pattern = vec![];
                    continue 'line_loop;
                }
                if current_pattern[up_index] != current_pattern[down_index] {
                    // this is not a perfect reflection, continue searching
                    break;
                }
            }
        }

        // try to find vertical reflection line
        fn is_column_not_equal(
            left_index: usize,
            right_index: usize,
            current_pattern: &Vec<&str>,
        ) -> bool {
            for i in 0..current_pattern.len() {
                let current_pattern_row = current_pattern[i].as_bytes();
                if current_pattern_row[left_index] != current_pattern_row[right_index] {
                    return true;
                }
            }
            false
        }
        let num_columns = current_pattern[0].len();
        for i in 1..num_columns {
            for j in 1.. {
                let (left_index, left_index_overflow) = i.overflowing_sub(j);
                let right_index = i + j - 1;
                if left_index_overflow || right_index == num_columns {
                    // reached end of valid reflection
                    sum += i;
                    current_pattern = vec![];
                    continue 'line_loop;
                }
                if is_column_not_equal(left_index, right_index, &current_pattern) {
                    // this is not a perfect reflection, continue searching
                    break;
                }
            }
        }

        unreachable!()
    }
    sum
}
