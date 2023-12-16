pub fn get_total_load_on_north_support_beam(input: &str) -> usize {
    let rows = input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();
    let num_rows = rows.len();
    let num_cols = rows[0].len();
    let mut current_load_in_col = vec![0; num_cols];
    let mut next_load_in_col = vec![num_rows; num_cols];
    for y in 0..num_rows {
        let current_row = rows[y];
        for x in 0..num_cols {
            match current_row[x] {
                b'.' => (),
                b'O' => {
                    current_load_in_col[x] += next_load_in_col[x];
                    next_load_in_col[x] -= 1;
                }
                b'#' => {
                    next_load_in_col[x] = num_rows - y - 1;
                }
                v => panic!("unexpected character {}", v),
            }
        }
    }
    current_load_in_col.into_iter().sum()
}
