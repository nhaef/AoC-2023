#![feature(ascii_char)]
use run_trait::AdventOfCodeSolution;
use state_machine::{find_a_to_z_steps, find_aaa_to_zzz_steps};

mod node;
mod state_machine;

pub struct Solution(usize);

impl AdventOfCodeSolution for Solution {
    fn name() -> &'static str {
        "--- Day 8: Haunted Wasteland ---"
    }

    fn input_1() -> &'static str {
        include_str!("../inputs/input_puzzle")
    }

    fn input_2() -> &'static str {
        include_str!("../inputs/input_puzzle")
    }

    fn input_1_example() -> &'static str {
        include_str!("../inputs/input_example_1")
    }

    fn input_2_example() -> &'static str {
        include_str!("../inputs/input_example_2")
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
