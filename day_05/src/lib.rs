use run_trait::AdventOfCodeSolution;
use seed_location_map::{
    get_lowest_seed_locations, get_lowest_seed_locations_reverse, read_seed_ranges, read_seeds,
};

mod seed_location_map;

pub struct Solution(pub i64);

impl AdventOfCodeSolution for Solution {
    fn name() -> &'static str {
        "--- Day 5: If You Give A Seed A Fertilizer ---"
    }

    fn input_1() -> &'static str {
        include_str!("../inputs/input_puzzle_1")
    }

    fn input_2() -> &'static str {
        include_str!("../inputs/input_puzzle_2")
    }

    fn input_1_example() -> &'static str {
        include_str!("../inputs/input_example")
    }

    fn input_2_example() -> &'static str {
        include_str!("../inputs/input_example")
    }

    fn solve_1(input: &str) -> Self {
        let mut lines = input.lines();
        let seeds = lines.next().expect("Could not read first line of input");
        let seeds = read_seeds(seeds);
        Self(get_lowest_seed_locations(seeds, lines))
    }

    fn solve_2(input: &str) -> Self {
        let mut lines = input.lines();
        let seed_ranges = lines.next().expect("Could not read first line of input");
        let seed_ranges = read_seed_ranges(seed_ranges);
        Self(get_lowest_seed_locations_reverse(seed_ranges, lines))
    }
}

impl std::fmt::Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The lowest location number is {}", self.0)
    }
}
