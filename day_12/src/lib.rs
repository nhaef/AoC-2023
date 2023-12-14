use aoc_trait::AdventOfCodeSolution;

mod record_1;
mod record_2;

pub struct Solution(usize);

impl AdventOfCodeSolution for Solution {
    fn name() -> &'static str {
        "--- Day 12: Hot Springs ---"
    }

    fn input_1() -> &'static str {
        include_str!("../inputs/input_puzzle")
    }

    fn input_2() -> &'static str {
        // include_str!("../inputs/input_puzzle")
        todo!()
    }

    fn input_1_example() -> &'static str {
        include_str!("../inputs/input_example")
    }

    fn input_2_example() -> &'static str {
        include_str!("../inputs/input_example")
    }

    fn solve_1(input: &str) -> Self {
        Self(record_1::get_sum_of_arrangements(input))
    }

    fn solve_2(input: &str) -> Self {
        Self(record_2::get_sum_of_arrangements(input))
    }
}

impl std::fmt::Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
