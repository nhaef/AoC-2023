use aoc_trait::AdventOfCodeSolution;
use seed_location_map::{
    get_lowest_seed_locations, get_lowest_seed_locations_reverse, read_seed_ranges, read_seeds,
};

mod seed_location_map;

pub struct Solution(i64);

impl AdventOfCodeSolution for Solution {
    fn day() -> usize {
        5
    }

    fn name() -> &'static str {
        "--- Day 5: If You Give A Seed A Fertilizer ---"
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
        write!(f, "the lowest location number is {}", self.0)
    }
}
