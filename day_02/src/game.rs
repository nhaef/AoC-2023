use crate::cube_set::CubeSet;

#[derive(Debug)]
pub struct Game {
    pub id: u32,
    revealed_cube_sets: Vec<CubeSet>,
}

impl Game {
    pub fn from_str(input_line: &str) -> Self {
        let (game_prefix, game_data) = match input_line.split_once(':') {
            None => panic!("Could not find delimiter ':' in input_line {}", input_line),
            Some(v) => v,
        };

        // Extract id from string like 'Game 42'
        let id = match game_prefix.split_once(' ') {
            None => panic!(
                "Could not find delimiter ' ' in game_prefix {}",
                game_prefix
            ),
            Some((_, id)) => id
                .parse::<u32>()
                .expect(&format!("Could not parse id {}", id)),
        };

        // Extract revealed_cube_sets from string like '1 red, 2 green; 4 red, 3 blue'
        let revealed_cube_sets = game_data.split(';').map(|s| CubeSet::from_str(s)).collect();

        Self {
            id,
            revealed_cube_sets,
        }
    }

    pub fn is_possible_with_bag(&self, bag: &CubeSet) -> bool {
        for revealed_cube_set in &self.revealed_cube_sets {
            if !bag.does_contains_enough_cubes(&revealed_cube_set) {
                return false;
            }
        }
        true
    }

    pub fn power_of_smallest_cube_set(self) -> u32 {
        CubeSet::get_smallest_cube_set(self.revealed_cube_sets).power()
    }
}
