const CHAR_GALAXY: char = '#';

#[derive(Debug)]
struct SpaceImageCoordinates(usize, usize);

pub struct SpaceImage {
    galaxies: Vec<SpaceImageCoordinates>,
}

impl SpaceImage {
    pub fn new(input: &str, expansion: usize) -> Self {
        let mut lines = input.lines().peekable();
        let mut galaxies = vec![];
        let mut empty_rows = vec![];
        let mut empty_rows_current = 0;
        let mut are_cols_empty = vec![true; lines.peek().unwrap().len()];
        for (y, line) in lines.enumerate() {
            let mut is_row_empty = true;
            for (x, char) in line.chars().enumerate() {
                match char {
                    CHAR_GALAXY => {
                        is_row_empty = false;
                        are_cols_empty[x] = false;
                        let galaxy_coordinates = SpaceImageCoordinates(x, y);
                        galaxies.push(galaxy_coordinates);
                    }
                    _ => (),
                }
            }
            if is_row_empty {
                empty_rows_current += expansion;
            }
            empty_rows.push(empty_rows_current);
        }
        let mut empty_cols = vec![];
        let mut empty_cols_current = 0;
        for is_col_empty in are_cols_empty {
            if is_col_empty {
                empty_cols_current += expansion;
            }
            empty_cols.push(empty_cols_current);
        }
        for galaxy_coordinates in galaxies.iter_mut() {
            galaxy_coordinates.0 += empty_cols[galaxy_coordinates.0];
            galaxy_coordinates.1 += empty_rows[galaxy_coordinates.1];
        }

        Self { galaxies }
    }

    pub fn get_sum_of_galaxy_distances(&self) -> usize {
        let mut sum = 0;
        for (galaxy_left, galaxy_right) in get_unique_pairs(&self.galaxies) {
            let delta_x = if galaxy_left.0 > galaxy_right.0 {
                galaxy_left.0 - galaxy_right.0
            } else {
                galaxy_right.0 - galaxy_left.0
            };
            let delta_y = if galaxy_left.1 > galaxy_right.1 {
                galaxy_left.1 - galaxy_right.1
            } else {
                galaxy_right.1 - galaxy_left.1
            };
            let distance = delta_x + delta_y;
            sum += distance;
        }
        sum
    }
}

fn get_unique_pairs<T>(values: &Vec<T>) -> Vec<(&T, &T)> {
    let length = values.len();
    let mut pairs = vec![];
    for i in 0..length {
        for j in (i + 1)..length {
            pairs.push((&values[i], &values[j]));
        }
    }
    pairs
}
