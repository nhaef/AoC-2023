use run_trait::AdventOfCodeSolution;

mod puzzle_1;
mod puzzle_2;

pub struct Solution(pub u32);

impl AdventOfCodeSolution for Solution {
    fn name() -> &'static str {
        "--- Day 1: Trebuchet?! ---"
    }

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

    fn solve_1(input: &str) -> Self {
        Self(puzzle_1::get_calibration_value_sum(input))
    }

    fn solve_2(input: &str) -> Self {
        Self(puzzle_2::get_calibration_value_sum(input))
    }
}

impl std::fmt::Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "calibration value: {}", self.0)
    }
}
