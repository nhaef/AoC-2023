use aoc_trait::AdventOfCodeSolution;
use oasis::{sum_and_extrapolate_next_values_for_input, sum_and_extrapolate_prev_values_for_input};

mod oasis;

pub enum SolutionType {
    Part1,
    Part2,
}

pub struct Solution(SolutionType, i64);

impl AdventOfCodeSolution for Solution {
    fn name() -> &'static str {
        "--- Day 9: Mirage Maintenance ---"
    }

    fn input_1() -> &'static str {
        include_str!("../inputs/input_puzzle")
    }

    fn input_2() -> &'static str {
        include_str!("../inputs/input_puzzle")
    }

    fn input_1_example() -> &'static str {
        include_str!("../inputs/input_example")
    }

    fn input_2_example() -> &'static str {
        include_str!("../inputs/input_example")
    }

    fn solve_1(input: &str) -> Self {
        Self(
            SolutionType::Part1,
            sum_and_extrapolate_next_values_for_input(input),
        )
    }

    fn solve_2(input: &str) -> Self {
        Self(
            SolutionType::Part2,
            sum_and_extrapolate_prev_values_for_input(input),
        )
    }
}

impl std::fmt::Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            SolutionType::Part1 => write!(f, "the sum of extrapolated next values is {}", self.1),
            SolutionType::Part2 => write!(f, "the sum of extrapolated prev values is {}", self.1),
        }
    }
}
