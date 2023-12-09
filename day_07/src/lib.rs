use aoc_trait::AdventOfCodeSolution;
use sort::{get_total_winnings_1, get_total_winnings_2, input_to_hands_2};

use crate::sort::input_to_hands_1;

mod hand;
mod sort;

pub struct Solution(u32);

impl AdventOfCodeSolution for Solution {
    fn name() -> &'static str {
        "--- Day 7: Camel Cards ---"
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
        Self(get_total_winnings_1(input_to_hands_1(input)))
    }

    fn solve_2(input: &str) -> Self {
        Self(get_total_winnings_2(input_to_hands_2(input)))
    }
}

impl std::fmt::Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The sum of all winnings is {}", self.0)
    }
}
