use run_trait::AdventOfCodeDay;
use seed_location_map::{
    get_lowest_seed_locations, get_lowest_seed_locations_reverse, read_seed_ranges, read_seeds,
};
use solution::Solution;

mod seed_location_map;
mod solution;

pub struct Day5;

impl AdventOfCodeDay<Solution> for Day5 {
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

    fn solve_1(input: &str) -> Solution {
        let mut lines = input.lines();
        let seeds = lines.next().expect("Could not read first line of input");
        let seeds = read_seeds(seeds);
        Solution(get_lowest_seed_locations(seeds, lines))
    }

    fn solve_2(input: &str) -> Solution {
        let mut lines = input.lines();
        let seed_ranges = lines.next().expect("Could not read first line of input");
        let seed_ranges = read_seed_ranges(seed_ranges);
        Solution(get_lowest_seed_locations_reverse(seed_ranges, lines))
    }
}
