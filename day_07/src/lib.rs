use aoc_trait::AdventOfCodeSolution;
use sort::{get_total_winnings_1, get_total_winnings_2, input_to_hands_2};

use crate::sort::input_to_hands_1;

mod hand;
mod sort;

pub struct Solution(u32);

impl AdventOfCodeSolution for Solution {
    fn day() -> usize {
        7
    }

    fn name() -> &'static str {
        "--- Day 7: Camel Cards ---"
    }

    fn solve_1(input: &str) -> Self {
        Self(get_total_winnings_1(input_to_hands_1(input)))
    }

    fn solve_2(input: &str) -> Self {
        Self(get_total_winnings_2(input_to_hands_2(input)))
    }
}

impl std::fmt::Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "the sum of all winnings is {}", self.0)
    }
}
