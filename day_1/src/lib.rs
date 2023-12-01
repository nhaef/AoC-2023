use run_trait::AdventOfCodeDay;
use solution::Solution;

mod puzzle_1;
mod puzzle_2;
mod solution;

pub struct Day1;

impl AdventOfCodeDay<Solution> for Day1 {
    fn input_1() -> &'static str {
        include_str!("../inputs/input_puzzle_1")
    }

    fn input_2() -> &'static str {
        include_str!("../inputs/input_puzzle_2")
    }

    fn input_1_example() -> &'static str {
        include_str!("../inputs/input_example_1")
    }

    fn input_2_example() -> &'static str {
        include_str!("../inputs/input_example_2")
    }
    
    fn solve_1(input: &str) -> Solution {
        Solution(puzzle_1::get_calibration_value_sum(input))
    }

    fn solve_2(input: &str) -> Solution {
        Solution(puzzle_2::get_calibration_value_sum(input))
    }
}
