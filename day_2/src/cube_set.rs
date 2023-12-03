#[derive(Debug)]
pub struct CubeSet {
    red_cubes: u32,
    green_cubes: u32,
    blue_cubes: u32,
}

impl CubeSet {
    pub fn new(red_cubes: u32, green_cubes: u32, blue_cubes: u32) -> Self {
        Self {
            red_cubes,
            green_cubes,
            blue_cubes,
        }
    }
    pub fn from_str(cube_set: &str) -> Self {
        let mut red_cubes = 0;
        let mut green_cubes = 0;
        let mut blue_cubes = 0;
        for count_color in cube_set.split(',') {
            let (count, color) = match count_color.rsplit_once(' ') {
                None => panic!(
                    "Could not find delimiter ' ' in count_color {}",
                    count_color
                ),
                Some((count, color)) => (
                    count.trim_start().parse::<u32>().expect(&format!(
                        "Could not parse count {} in {}",
                        count, count_color
                    )),
                    color,
                ),
            };
            match color {
                "red" => red_cubes = count,
                "green" => green_cubes = count,
                "blue" => blue_cubes = count,
                _ => panic!("Unknown color '{}'", color),
            }
        }
        Self {
            red_cubes,
            green_cubes,
            blue_cubes,
        }
    }

    pub fn does_contains_enough_cubes(&self, other: &CubeSet) -> bool {
        other.red_cubes <= self.red_cubes
            && other.green_cubes <= self.green_cubes
            && other.blue_cubes <= self.blue_cubes
    }

    pub fn power(&self) -> u32 {
        self.red_cubes * self.green_cubes * self.blue_cubes
    }

    pub fn get_smallest_cube_set<C>(cubes: C) -> CubeSet
    where
        C: IntoIterator<Item = CubeSet>,
    {
        cubes.into_iter().fold(
            CubeSet::new(0, 0, 0),
            |mut smallest_cube_set, current_cube_set| {
                smallest_cube_set.red_cubes =
                    u32::max(smallest_cube_set.red_cubes, current_cube_set.red_cubes);
                smallest_cube_set.green_cubes =
                    u32::max(smallest_cube_set.green_cubes, current_cube_set.green_cubes);
                smallest_cube_set.blue_cubes =
                    u32::max(smallest_cube_set.blue_cubes, current_cube_set.blue_cubes);
                smallest_cube_set
            },
        )
    }
}
