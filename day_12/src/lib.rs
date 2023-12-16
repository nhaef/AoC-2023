use aoc_trait::AdventOfCodeSolution;

mod record_1;
mod record_2;

pub struct Solution(usize);

impl AdventOfCodeSolution for Solution {
    fn day() -> usize {
        12
    }

    fn name() -> &'static str {
        "--- Day 12: Hot Springs ---"
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
        write!(f, "the sum of possible arrangement counts is {}", self.0)
    }
}
