use aoc_trait::AdventOfCodeSolution;
use state_machine::{find_a_to_z_steps, find_aaa_to_zzz_steps};

mod node;
mod state_machine;

pub struct Solution(usize);

impl AdventOfCodeSolution for Solution {
    fn day() -> usize {
        8
    }

    fn name() -> &'static str {
        "--- Day 8: Haunted Wasteland ---"
    }

    fn solve_1(input: &str) -> Self {
        Self(find_aaa_to_zzz_steps(input))
    }

    fn solve_2(input: &str) -> Self {
        Self(find_a_to_z_steps(input))
    }
}

impl std::fmt::Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "the amount of required steps is {}", self.0)
    }
}
