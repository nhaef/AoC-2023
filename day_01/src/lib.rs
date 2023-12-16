use aoc_trait::AdventOfCodeSolution;

mod puzzle_1;
mod puzzle_2;

pub struct Solution(u32);

impl AdventOfCodeSolution for Solution {
    fn day() -> usize {
        1
    }

    fn name() -> &'static str {
        "--- Day 1: Trebuchet?! ---"
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
        write!(f, "the calibration value is {}", self.0)
    }
}
